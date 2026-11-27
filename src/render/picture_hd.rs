//! VERA-only, experimental (`docs/picture-model-art.md`, "Memory"): the
//! HD twins of the sprite atlas's picture art above 1x, made when the zoomed
//! draw needs them.
//!
//! The atlas keeps only the game-pixel pages of its pictures. Each of their
//! sprites above 1x registers an [`HdTwin`], the picture's files, frame,
//! rectangle and house, here; a frame without a drawn house-colour pixel
//! registers one twin for every house. When the zoomed draw
//! (`app::presentation::render::picture_zoom`) meets a sprite whose twin is
//! not on the GPU it asks for it and draws the magnified 1x pixels meanwhile;
//! a worker decodes the picture one frame at a time and makes the twin and
//! its mips, and the next frames upload what is ready, a bounded amount a
//! frame, into HD pages of their own, created when first needed. The pages
//! stay within [`HD_BUDGET_BYTES`]: the page used longest ago goes first,
//! and all go once nothing has been magnified for [`IDLE_FRAMES`] frames.
//! Its twins come back when drawn again. Nothing here reaches the
//! simulation, and with no picture above 1x (retail data) nothing here
//! holds or does anything.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, mpsc};

use crate::render::picture_atlas::{HdFiles, HdTwin, mip_chain, mip_levels};
use crate::render::picture_zoom::PictureHdPage;
use crate::rules::house_colors::HouseColorIndex;

/// The most the HD pages may hold on the GPU, colour with its mips and
/// index planes together.
pub(crate) const HD_BUDGET_BYTES: u64 = 512 << 20;
/// An HD page's side in texels; a twin larger than that gets a page of its
/// own.
const HD_PAGE_SIDE: u32 = 2048;
/// Level-0 colour bytes uploaded in one frame, beyond the first twin.
const UPLOAD_BYTES_PER_FRAME: usize = 32 << 20;
/// Twins asked of the workers and not uploaded yet, so what waits for an
/// upload stays bounded.
const TWINS_IN_FLIGHT: usize = 32;
/// Frames without a magnified picture after which every HD page goes.
pub(crate) const IDLE_FRAMES: u32 = 1800;

type TwinId = u32;

/// What makes two requests the same twin: the file the 1x frames came from
/// (never the name as declared, which the theater lookup may replace), its
/// scale, the frame, the 1x rectangle and the house.
type TwinKey = (String, u32, u32, [u32; 4], Option<HouseColorIndex>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TwinState {
    /// Not on the GPU and not asked for.
    Missing,
    /// Asked for this frame.
    Wanted,
    /// With a worker, or decoded and waiting for its upload.
    Queued,
    /// On HD page `page` at `origin` texels.
    Resident { page: usize, origin: [u32; 2] },
    /// Its picture cannot be decoded again or its page exceeds the budget:
    /// it keeps its 1x pixels.
    Failed,
}

struct Twin {
    request: HdTwin,
    state: TwinState,
    /// The frame it was last drawn.
    last_used: u64,
}

/// What a worker made of one twin: its premultiplied colour with its mips,
/// level 0 first, and its index plane; None when the picture failed.
struct Decoded {
    twin: TwinId,
    pixels: Option<(u32, [u32; 2], Vec<u8>, Vec<u8>)>,
}

/// One HD page's place keeping: the scale of its art, its size and mip
/// levels, the shelf it fills, its twins and the frame it was last drawn.
struct PageBook {
    scale: u32,
    size: [u32; 2],
    levels: u32,
    /// Next x, shelf top, shelf height, in texels.
    shelf: [u32; 3],
    twins: Vec<TwinId>,
    last_used: u64,
    bytes: u64,
}

impl PageBook {
    /// The alignment twin origins keep, so each mip level starts on a texel.
    fn align(&self) -> u32 {
        1 << (self.levels - 1)
    }

    /// Shelf placement for a `size` twin, one game pixel (`scale` texels)
    /// from its neighbours so filtering stays inside each twin.
    fn place(&mut self, size: [u32; 2]) -> Option<[u32; 2]> {
        let align = self.align();
        let up = |value: u32| value.div_ceil(align) * align;
        let gap = self.scale;
        let [x, top, row] = &mut self.shelf;
        if *x + size[0] > self.size[0] {
            *top = up(*top + *row + gap);
            *x = 0;
            *row = 0;
        }
        if *x + size[0] > self.size[0] || *top + size[1] > self.size[1] {
            return None;
        }
        let origin = [*x, *top];
        *x = up(*x + size[0] + gap);
        *row = (*row).max(size[1]);
        Some(origin)
    }
}

/// The GPU side of an HD page.
struct GpuPage {
    color: wgpu::Texture,
    indices: wgpu::Texture,
    sampled: PictureHdPage,
}

/// Bytes an HD page of `size` texels and `levels` mips holds on the GPU.
fn page_bytes(size: [u32; 2], levels: u32) -> u64 {
    let color: u64 = (0..levels)
        .map(|level| {
            let [w, h] = size.map(|side| u64::from((side >> level).max(1)));
            w * h * 4
        })
        .sum();
    color + u64::from(size[0]) * u64::from(size[1])
}

/// The atlas's picture HD twins: their requests, where those resident are,
/// and the HD pages.
#[derive(Default)]
pub(crate) struct PictureHdStore {
    twins: Vec<Twin>,
    /// One twin per resolved file, scale, frame, rectangle and house.
    by_key: HashMap<TwinKey, TwinId>,
    /// Atlas entry (page, UV origin bits) → its twin and UV size.
    entries: HashMap<(usize, u32, u32), (TwinId, [f32; 2])>,
    /// Atlas pages with an entry above.
    twin_pages: HashSet<usize>,
    pages: Vec<Option<(PageBook, Option<GpuPage>)>>,
    resident_bytes: u64,
    wanted: Vec<TwinId>,
    in_flight: usize,
    channel: Option<(mpsc::Sender<Vec<Decoded>>, mpsc::Receiver<Vec<Decoded>>)>,
    ready: std::collections::VecDeque<Decoded>,
    frame: u64,
    idle_frames: u32,
    budget: u64,
    /// Twins the current magnified frame drew that were not on the GPU.
    unready: usize,
}

impl PictureHdStore {
    /// Whether any atlas sprite has an HD twin.
    pub(crate) fn has_twins(&self) -> bool {
        !self.twin_pages.is_empty()
    }

    /// Register the atlas entry on `page` at `uv_origin`, `uv_size` as drawn
    /// from `twin`. Twins of the same file, frame, rectangle and house are
    /// one.
    pub(crate) fn register(
        &mut self,
        page: usize,
        uv_origin: [f32; 2],
        uv_size: [f32; 2],
        twin: HdTwin,
    ) {
        let key = (
            twin.files.file.clone(),
            twin.files.scale,
            twin.frame,
            [twin.origin[0], twin.origin[1], twin.size[0], twin.size[1]],
            twin.house.as_ref().map(|(house, _)| *house),
        );
        let next = self.twins.len() as TwinId;
        let id = *self.by_key.entry(key).or_insert_with(|| {
            self.twins.push(Twin {
                request: twin,
                state: TwinState::Missing,
                last_used: 0,
            });
            next
        });
        self.entries.insert(
            (page, uv_origin[0].to_bits(), uv_origin[1].to_bits()),
            (id, uv_size),
        );
        self.twin_pages.insert(page);
    }

    /// The HD page and UV rectangle (origin, size) that draw the instance
    /// at `uv_origin`, `uv_size` of atlas page `page` at its own resolution;
    /// None when its twin is not on the GPU, which asks for it. Marks the
    /// page drawn this frame.
    pub(crate) fn lookup(
        &mut self,
        page: usize,
        uv_origin: [f32; 2],
        uv_size: [f32; 2],
    ) -> Option<(usize, [f32; 4])> {
        if !self.twin_pages.contains(&page) {
            return None;
        }
        let &(id, entry_size) =
            self.entries
                .get(&(page, uv_origin[0].to_bits(), uv_origin[1].to_bits()))?;
        let twin = &mut self.twins[id as usize];
        twin.last_used = self.frame;
        if !matches!(twin.state, TwinState::Resident { .. } | TwinState::Failed) {
            // Not drawn sharp this frame: the store has not settled.
            self.unready += 1;
        }
        match twin.state {
            TwinState::Resident { page, origin } => {
                let (book, _) = self.pages[page].as_mut()?;
                book.last_used = self.frame;
                let hd_size = twin.request.size.map(|side| side * book.scale);
                let page_size = book.size.map(|side| side as f32);
                // The instance's share of its entry, on the twin.
                let share = [uv_size[0] / entry_size[0], uv_size[1] / entry_size[1]];
                Some((
                    page,
                    [
                        origin[0] as f32 / page_size[0],
                        origin[1] as f32 / page_size[1],
                        hd_size[0] as f32 * share[0] / page_size[0],
                        hd_size[1] as f32 * share[1] / page_size[1],
                    ],
                ))
            }
            TwinState::Missing => {
                twin.state = TwinState::Wanted;
                self.wanted.push(id);
                None
            }
            TwinState::Wanted | TwinState::Queued | TwinState::Failed => None,
        }
    }

    /// HD page `page`'s textures, while it exists.
    pub(crate) fn page(&self, page: usize) -> Option<&PictureHdPage> {
        let (_, gpu) = self.pages.get(page)?.as_ref()?;
        gpu.as_ref().map(|gpu| &gpu.sampled)
    }

    fn budget(&self) -> u64 {
        if self.budget == 0 {
            HD_BUDGET_BYTES
        } else {
            self.budget
        }
    }

    /// Start a magnified frame: take in what the workers made, for
    /// [`Self::end_frame`] to upload once this frame's lookups have said
    /// which pages it draws.
    pub(crate) fn begin_frame(&mut self) {
        self.frame += 1;
        self.idle_frames = 0;
        self.unready = 0;
        if let Some((_, receiver)) = &self.channel {
            while let Ok(batch) = receiver.try_recv() {
                self.ready.extend(batch);
            }
        }
    }

    /// End a magnified frame: upload what the workers made, at most
    /// [`UPLOAD_BYTES_PER_FRAME`] beyond the first twin, the next frame
    /// drawing it, then hand the twins this frame asked for to the workers
    /// ([`Self::dispatch`]).
    pub(crate) fn end_frame(&mut self, device: Option<(&wgpu::Device, &wgpu::Queue)>) {
        let mut uploaded = 0;
        while uploaded == 0 || uploaded < UPLOAD_BYTES_PER_FRAME {
            let Some(decoded) = self.ready.pop_front() else {
                break;
            };
            self.in_flight = self.in_flight.saturating_sub(1);
            uploaded += self.upload(device, decoded).max(1);
        }
        self.dispatch();
    }

    /// Place and upload one decoded twin; the level-0 bytes it uploaded.
    fn upload(&mut self, device: Option<(&wgpu::Device, &wgpu::Queue)>, decoded: Decoded) -> usize {
        let id = decoded.twin as usize;
        if self.twins[id].state != TwinState::Queued {
            return 0;
        }
        let Some((levels, size, mips, indices)) = decoded.pixels else {
            self.twins[id].state = TwinState::Failed;
            return 0;
        };
        let scale = self.twins[id].request.files.scale;
        let Some((page, origin)) = self.allocate(device.map(|(device, _)| device), scale, size)
        else {
            self.twins[id].state = TwinState::Missing;
            return 0;
        };
        self.twins[id].state = TwinState::Resident { page, origin };
        let Some((book, gpu)) = self.pages[page].as_mut() else {
            return 0;
        };
        book.twins.push(decoded.twin);
        if let (Some(gpu), Some((_, queue))) = (gpu.as_ref(), device) {
            let mut start = 0;
            for level in 0..levels.min(book.levels) {
                let at = origin.map(|side| side >> level);
                let extent = size.map(|side| (side >> level).max(1));
                let len = (extent[0] * extent[1] * 4) as usize;
                let Some(texels) = mips.get(start..start + len) else {
                    break;
                };
                write_level(queue, &gpu.color, level, at, extent, 4, texels);
                start += len;
            }
            write_level(queue, &gpu.indices, 0, origin, size, 1, &indices);
        }
        (size[0] * size[1] * 4) as usize
    }

    /// A place for a `size` twin of `scale` art: on a page of that scale
    /// with room, else on a new page, the pages used longest ago (not this
    /// frame) going first while the budget has no room for it. None when it
    /// cannot fit within the budget.
    fn allocate(
        &mut self,
        device: Option<&wgpu::Device>,
        scale: u32,
        size: [u32; 2],
    ) -> Option<(usize, [u32; 2])> {
        for (index, page) in self.pages.iter_mut().enumerate() {
            if let Some((book, _)) = page
                && book.scale == scale
                && let Some(origin) = book.place(size)
            {
                return Some((index, origin));
            }
        }
        let levels = mip_levels(scale, [HD_PAGE_SIDE; 2]);
        let align = 1u32 << (levels - 1);
        let page_size = size
            .map(|side| side.div_ceil(align).max(1) * align)
            .map(|side| side.max(HD_PAGE_SIDE));
        if let Some(device) = device {
            let max = device.limits().max_texture_dimension_2d;
            if page_size[0] > max || page_size[1] > max {
                return None;
            }
        }
        let bytes = page_bytes(page_size, levels);
        while self.resident_bytes + bytes > self.budget() {
            let victim = self.victim()?;
            log::info!("Picture HD page {victim} dropped for room");
            self.evict(victim);
        }
        let gpu = device.map(|device| create_page(device, page_size, levels));
        let mut book = PageBook {
            scale,
            size: page_size,
            levels,
            shelf: [0; 3],
            twins: Vec::new(),
            last_used: self.frame,
            bytes,
        };
        let origin = book.place(size)?;
        self.resident_bytes += bytes;
        let slot = self.pages.iter().position(Option::is_none);
        let entry = Some((book, gpu));
        let index = match slot {
            Some(index) => {
                self.pages[index] = entry;
                index
            }
            None => {
                self.pages.push(entry);
                self.pages.len() - 1
            }
        };
        Some((index, origin))
    }

    /// The page to drop for room: the one drawn longest ago, else, when
    /// every page was drawn this frame, the one with the fewest twins drawn
    /// this frame among those holding twins that were not, so pages that
    /// keep one visible twin among many stale ones do not hold the budget
    /// for ever. None when every twin on every page was drawn this frame.
    fn victim(&self) -> Option<usize> {
        let pages = || {
            self.pages
                .iter()
                .enumerate()
                .filter_map(|(index, page)| Some((index, &page.as_ref()?.0)))
        };
        if let Some((index, _)) = pages()
            .filter(|(_, book)| book.last_used < self.frame)
            .min_by_key(|(_, book)| book.last_used)
        {
            return Some(index);
        }
        pages()
            .filter_map(|(index, book)| {
                let drawn = book
                    .twins
                    .iter()
                    .filter(|&&id| self.twins[id as usize].last_used >= self.frame)
                    .count();
                (drawn < book.twins.len()).then_some((index, drawn))
            })
            .min_by_key(|&(_, drawn)| drawn)
            .map(|(index, _)| index)
    }

    /// Whether every twin the last magnified frame drew was on the GPU (or
    /// cannot be made), with nothing asked for still on its way: a capture
    /// of the next frame shows the art sharp wherever it can be.
    pub(crate) fn settled(&self) -> bool {
        self.unready == 0 && self.in_flight == 0 && self.ready.is_empty()
    }

    /// Drop HD page `page`; its twins are missing again.
    fn evict(&mut self, page: usize) {
        let Some((book, _)) = self.pages.get_mut(page).and_then(Option::take) else {
            return;
        };
        self.resident_bytes -= book.bytes;
        for id in book.twins {
            self.twins[id as usize].state = TwinState::Missing;
        }
    }

    /// Hand the twins asked for this frame to the workers, up to
    /// [`TWINS_IN_FLIGHT`] waiting at once, one job per picture so each is
    /// decoded once. The rest are asked for again when drawn again.
    fn dispatch(&mut self) {
        let wanted = std::mem::take(&mut self.wanted);
        // No room: the budget is full and no page can go ([`Self::victim`]).
        let full = self.resident_bytes + page_bytes([HD_PAGE_SIDE; 2], 1) > self.budget()
            && self.victim().is_none();
        let room = if full {
            0
        } else {
            TWINS_IN_FLIGHT.saturating_sub(self.in_flight)
        };
        let mut jobs: Vec<(Arc<HdFiles>, Vec<(TwinId, HdTwin)>)> = Vec::new();
        for (taken, id) in wanted.into_iter().enumerate() {
            let twin = &mut self.twins[id as usize];
            if taken >= room {
                twin.state = TwinState::Missing;
                continue;
            }
            twin.state = TwinState::Queued;
            self.in_flight += 1;
            let request = twin.request.clone();
            match jobs
                .iter_mut()
                .find(|(files, _)| Arc::ptr_eq(files, &request.files))
            {
                Some((_, twins)) => twins.push((id, request)),
                None => jobs.push((Arc::clone(&request.files), vec![(id, request)])),
            }
        }
        if jobs.is_empty() {
            return;
        }
        let (sender, _) = self.channel.get_or_insert_with(mpsc::channel);
        for (files, twins) in jobs {
            let sender = sender.clone();
            crate::sim::parallel::spawn_on_pool(move || {
                // The atlas may be gone; its twins are then not wanted.
                let _ = sender.send(decode_twins(&files, twins));
            });
        }
    }

    /// A frame with nothing magnified: after [`IDLE_FRAMES`] of them every
    /// HD page goes.
    pub(crate) fn idle(&mut self) {
        if self.pages.is_empty() {
            return;
        }
        self.idle_frames += 1;
        if self.idle_frames >= IDLE_FRAMES {
            for page in 0..self.pages.len() {
                self.evict(page);
            }
            self.pages.clear();
        }
    }
}

/// Make each of `twins` of `files` at its own resolution, decoding the
/// picture once, a frame at a time, up to the last frame asked for.
fn decode_twins(files: &HdFiles, mut twins: Vec<(TwinId, HdTwin)>) -> Vec<Decoded> {
    twins.sort_by_key(|(_, twin)| twin.frame);
    let last = twins.last().map_or(0, |(_, twin)| twin.frame as usize);
    let mut out = Vec::with_capacity(twins.len());
    let mut next = 0;
    let decoded = files.each_frame(last, |index, rgba, mask, width| {
        while let Some((id, twin)) = twins
            .get(next)
            .filter(|(_, twin)| twin.frame as usize == index)
        {
            let sprite = crate::render::picture_atlas::hd_sprite(
                rgba,
                mask,
                width,
                files.scale,
                twin.origin,
                twin.size,
                twin.house.as_ref().map(|(_, ramp)| ramp),
            );
            let levels = mip_levels(sprite.scale, sprite.size);
            out.push(Decoded {
                twin: *id,
                pixels: Some((
                    levels,
                    sprite.size,
                    mip_chain(sprite.rgba, sprite.size, levels),
                    sprite.indices,
                )),
            });
            next += 1;
        }
    });
    if let Err(error) = decoded {
        log::warn!(
            "[{}] {}: its HD frames cannot be decoded ({error}); they draw at 1x",
            files.source.section,
            files.file
        );
    }
    // A frame the picture does not reach, or one after a failure.
    out.extend(twins[next..].iter().map(|(id, _)| Decoded {
        twin: *id,
        pixels: None,
    }));
    out
}

fn create_page(device: &wgpu::Device, size: [u32; 2], levels: u32) -> GpuPage {
    let texture = |label, format, mip_level_count| {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        })
    };
    // Encoded bytes, as the palette rows (`PictureHdPage::new`).
    let color = texture("Picture HD page", wgpu::TextureFormat::Rgba8Unorm, levels);
    let indices = texture("Picture HD page indices", wgpu::TextureFormat::R8Uint, 1);
    let sampled = PictureHdPage::from_views(
        color.create_view(&Default::default()),
        indices.create_view(&Default::default()),
    );
    GpuPage {
        color,
        indices,
        sampled,
    }
}

/// Upload one tightly packed rectangle of texels into mip `level` of
/// `texture` at `origin`.
fn write_level(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    level: u32,
    origin: [u32; 2],
    size: [u32; 2],
    bytes_per_texel: u32,
    texels: &[u8],
) {
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: level,
            origin: wgpu::Origin3d {
                x: origin[0],
                y: origin[1],
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        texels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(size[0] * bytes_per_texel),
            rows_per_image: Some(size[1]),
        },
        wgpu::Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        },
    );
}

#[cfg(all(test, tests_render_a_r))]
mod tests {
    use super::*;
    use crate::assets::pal_file::Color;
    use crate::render::picture_atlas::PictureSource;
    use crate::render::picture_atlas::tests::decoded;
    use crate::render::picture_atlas::{LIT_INDEX, RAMP_SIZE, UNLIT_INDEX};

    const RED: [u8; 4] = [200, 0, 0, 255];
    const NONE: [u8; 4] = [0; 4];
    const HOUSE: [u8; 4] = [255, 0, 0, 255];

    fn ramp() -> [Color; RAMP_SIZE] {
        std::array::from_fn(|step| Color::rgb(0, step as u8, 0))
    }

    /// A twin asked for is made from the picture's file with the pixels the
    /// atlas built eagerly before (`hd_sprites_keep_the_crop...`): the 1x
    /// crop at picture resolution, premultiplied holes, unlit indices.
    #[test]
    fn an_hd_request_makes_the_pixels_the_eager_twin_had() {
        // Canvas 2x1 of a 4x2 picture: the left game pixel has three of four
        // pixels drawn (drawn), the right one has one (a hole at 1x).
        let rows = [[RED, NONE, RED, NONE], [RED, RED, NONE, NONE]];
        let unlit = [0, 255, 0, 255];
        let mask_rows = [[[0; 4]; 4], [[0; 4], unlit, [0; 4], [0; 4]]];
        let frames = decoded(
            4,
            2,
            &[rows.concat().concat()],
            Some(&[mask_rows.concat().concat()]),
            [2, 1],
        );
        let sprite = frames.sprite(0, None).unwrap();
        let twin = frames.hd_twin(0, &sprite, None).unwrap();
        let files = Arc::clone(&twin.files);
        let made = decode_twins(&files, vec![(7, twin)]);
        let [
            Decoded {
                twin: 7,
                pixels: Some((levels, size, mips, indices)),
            },
        ] = &made[..]
        else {
            panic!("one twin made");
        };
        assert_eq!((*levels, *size), (2, [2, 2]));
        assert_eq!(mips[..16], [RED, NONE, RED, RED].concat());
        assert_eq!(indices[..], [LIT_INDEX, 0, LIT_INDEX, UNLIT_INDEX]);
    }

    /// A `NewTheater=yes` picture's twin is made from the file its 1x frames
    /// came from: on a temperate map `NTNRCT.png` (here a link to the generic
    /// `NGNRCT.png`), never the declared `NANRCT.png` (the snow variant).
    #[test]
    fn a_theater_substituted_picture_makes_its_twin_from_the_substituted_file() {
        const GREEN: [u8; 4] = [0, 200, 0, 255];
        const WHITE: [u8; 4] = [250, 250, 250, 255];
        let root = std::env::temp_dir().join(format!("picture-hd-theater-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let png = |color: [u8; 4]| {
            crate::assets::picture_file::encode_frames(2, 2, &[color.repeat(4)]).unwrap()
        };
        std::fs::write(root.join("NGNRCT.png"), png(GREEN)).unwrap();
        std::fs::write(root.join("NANRCT.png"), png(WHITE)).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(root.join("NGNRCT.png"), root.join("NTNRCT.png")).unwrap();
        #[cfg(not(unix))]
        std::fs::copy(root.join("NGNRCT.png"), root.join("NTNRCT.png")).unwrap();
        let assets = crate::assets::asset_manager::AssetManager::from_loose_root_for_test(&root);
        let art = crate::rules::art_data::ArtRegistry::from_ini(
            &crate::rules::ini_parser::IniFile::from_str(
                "[NANRCT]\nNewTheater=yes\nPicture=NANRCT.png\nPictureSize=1,1\n",
            ),
        );
        let source = PictureSource {
            section: "NANRCT".into(),
            decl: art.get("NANRCT").unwrap().picture.clone().unwrap(),
        };
        let mut pictures = crate::render::picture_atlas::PictureSources::default();
        let frames = pictures
            .frames(&source, &art, &assets, "TEMPERATE")
            .unwrap();
        let sprite = frames.sprite(0, None).unwrap();
        assert_eq!(sprite.rgba, GREEN, "the 1x frame is the temperate file's");
        let twin = frames.hd_twin(0, &sprite, None).unwrap();
        assert_eq!(twin.files.file, "NTNRCT.png");
        let files = Arc::clone(&twin.files);
        let made = decode_twins(&files, vec![(0, twin)]);
        let _ = std::fs::remove_dir_all(&root);
        let [
            Decoded {
                pixels: Some((_, _, mips, _)),
                ..
            },
        ] = &made[..]
        else {
            panic!("one twin made");
        };
        assert_eq!(
            mips[..16],
            GREEN.repeat(4)[..],
            "the twin is the temperate file's too"
        );
    }

    /// Houses share the twin of a frame with no house-colour pixel at the
    /// picture's resolution, and get their own of a frame with one, even a
    /// pixel too few to colour the game pixel.
    #[test]
    fn houses_share_the_twin_of_a_frame_without_house_colour() {
        // Frame 0 plain; frame 1 has one house pixel of four.
        let frame = [RED, RED, RED, RED].concat();
        let plain = vec![0; 16];
        let one_house = [HOUSE, NONE, NONE, NONE].concat();
        let frames = decoded(
            2,
            2,
            &[frame.clone(), frame],
            Some(&[plain, one_house]),
            [1, 1],
        );
        let ramp = ramp();
        let mut store = PictureHdStore::default();
        for frame in 0..2 {
            let sprite = frames.sprite(frame, Some(&ramp)).unwrap();
            for house in 0..2u8 {
                let twin = frames
                    .hd_twin(frame, &sprite, Some((HouseColorIndex(house), &ramp)))
                    .unwrap();
                let u = f32::from(house) / 4.0 + frame as f32 / 2.0;
                store.register(0, [u, 0.0], [0.25, 1.0], twin);
            }
        }
        assert_eq!(store.twins.len(), 3, "one plain twin, one per house");
        assert!(store.twins[0].request.house.is_none());
    }

    /// Pages within the budget: a new page evicts the one drawn longest ago,
    /// never one drawn this frame, and a twin that finds no room stays
    /// missing; with nothing magnified for a while every page goes.
    #[test]
    fn hd_pages_evict_the_least_recently_drawn_within_the_budget() {
        let frames = decoded(
            2,
            2,
            &[[HOUSE; 4].concat()],
            Some(&[[HOUSE; 4].concat()]),
            [1, 1],
        );
        let sprite = frames.sprite(0, None).unwrap();
        let levels = mip_levels(2, [HD_PAGE_SIDE; 2]);
        let mut store = PictureHdStore {
            budget: 2 * page_bytes([HD_PAGE_SIDE; 2], levels),
            ..Default::default()
        };
        let ramp = ramp();
        for house in 0..4u8 {
            let twin = frames
                .hd_twin(0, &sprite, Some((HouseColorIndex(house), &ramp)))
                .unwrap();
            store.register(0, [f32::from(house) / 4.0, 0.0], [0.25, 1.0], twin);
        }
        let upload = |store: &mut PictureHdStore, twin: TwinId| {
            // Asked for by this frame's draw.
            store.twins[twin as usize].state = TwinState::Queued;
            store.twins[twin as usize].last_used = store.frame;
            let whole_page = (levels, [HD_PAGE_SIDE; 2], Vec::new(), Vec::new());
            store.upload(
                None,
                Decoded {
                    twin,
                    pixels: Some(whole_page),
                },
            );
            store.twins[twin as usize].state
        };
        store.begin_frame();
        upload(&mut store, 0);
        upload(&mut store, 1);
        store.begin_frame();
        // Twin 1 is drawn this frame; twin 0's page is the oldest.
        assert!(store.lookup(0, [0.25, 0.0], [0.25, 1.0]).is_some());
        assert_eq!(
            upload(&mut store, 2),
            TwinState::Resident {
                page: 0,
                origin: [0, 0]
            }
        );
        assert_eq!(store.twins[0].state, TwinState::Missing);
        assert_eq!(upload(&mut store, 3), TwinState::Missing, "no room");
        for _ in 0..IDLE_FRAMES {
            store.idle();
        }
        assert_eq!((store.resident_bytes, store.pages.len()), (0, 0));
        assert_eq!(store.twins[1].state, TwinState::Missing);
    }

    /// A page that keeps one twin on screen among stale ones does not hold
    /// the budget: with no page left undrawn, the one with the fewest twins
    /// drawn this frame goes, and the store settles once nothing drawn is
    /// missing or on its way.
    #[test]
    fn a_page_of_mostly_stale_twins_gives_way_when_the_budget_is_full() {
        let frames = decoded(
            2,
            2,
            &[[HOUSE; 4].concat()],
            Some(&[[HOUSE; 4].concat()]),
            [1, 1],
        );
        let sprite = frames.sprite(0, None).unwrap();
        let levels = mip_levels(2, [HD_PAGE_SIDE; 2]);
        let mut store = PictureHdStore {
            budget: page_bytes([HD_PAGE_SIDE; 2], levels),
            ..Default::default()
        };
        let ramp = ramp();
        for house in 0..3u8 {
            let twin = frames
                .hd_twin(0, &sprite, Some((HouseColorIndex(house), &ramp)))
                .unwrap();
            store.register(0, [f32::from(house) / 4.0, 0.0], [0.25, 1.0], twin);
        }
        // Two 1000x2000 twins fill a 2048 page, with the gap between.
        let upload = |store: &mut PictureHdStore, twin: TwinId| {
            store.twins[twin as usize].state = TwinState::Queued;
            store.twins[twin as usize].last_used = store.frame;
            let quarter = (levels, [1000, 2000], Vec::new(), Vec::new());
            store.upload(
                None,
                Decoded {
                    twin,
                    pixels: Some(quarter),
                },
            );
            store.twins[twin as usize].state
        };
        store.begin_frame();
        upload(&mut store, 0);
        upload(&mut store, 1);
        store.begin_frame();
        // Only twin 1 is on screen now, and twin 2 comes into view.
        assert!(store.lookup(0, [0.25, 0.0], [0.25, 1.0]).is_some());
        assert!(store.lookup(0, [0.5, 0.0], [0.25, 1.0]).is_none());
        assert!(!store.settled(), "twin 2 is missing");
        assert!(matches!(
            upload(&mut store, 2),
            TwinState::Resident { page: 0, .. }
        ));
        assert_eq!(store.twins[0].state, TwinState::Missing);
        store.wanted.clear();
        store.begin_frame();
        assert!(store.lookup(0, [0.5, 0.0], [0.25, 1.0]).is_some());
        assert!(store.settled());
    }
}
