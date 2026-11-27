//! VERA's HD art terrain: 2x upscaled theater tiles drawn over the
//! wheel-zoomed world.
//!
//! VERA-only presentation extension, part of HD art (`[VERA] HdArt=`, `render::hd_switch`) (off by
//! default). The world is still composed at native pixels, exactly as before;
//! when the wheel zoom magnifies it 1.5x or more, this pass replaces the pixels
//! of the zoomed copy that the native world still shows as a tile's own colour
//! with that tile's HD picture (`hd_terrain.wgsl`), under the light the native
//! pixel received. Nothing reaches `sim/`, a save, a replay or the network.
//!
//! The pictures live in the art pack's `terrain` folder
//! (`hd_art/pack/terrain` beside the executable, or `VERA20K_HD_ART_PACK`):
//! `<hash>.png` per tile, exactly 2x the tile's rectangle, RGB in the
//! theater palette. The hash ([`tile_hash`]) is the FNV-1a of the tile's size
//! and the RGBA the engine colours it with (`TmpFile::tile_to_rgba`), so a
//! tile finds its picture whatever map, set or variant uses it.
//! `VERA20K_HD_DUMP_TERRAIN=<folder>` writes `<hash>.rgba` (u16 width,
//! u16 height, RGBA) for every tile of the loaded theater the pack lacks, for
//! an offline upscaler.

use std::collections::HashMap;
use std::path::PathBuf;

use bytemuck::{Pod, Zeroable};

/// HD pixels per side of the page.
const PAGE: u32 = 4096;

/// New pack files read in one frame: a screen of ground arrives over a few
/// frames instead of stalling one.
const LOADS_PER_FRAME: usize = 96;

/// FNV-1a 64 of a tile: width and height (u16 little-endian), then its RGBA.
pub(crate) fn tile_hash(width: u32, height: u32, rgba: &[u8]) -> u64 {
    super::hd_art::frame_hash(width, height, rgba)
}

/// The pack's terrain folder (`hd_art::pack_dir`).
fn pack_dir() -> Option<PathBuf> {
    super::hd_art::pack_subdir("terrain")
}

/// Write every tile of `images` the pack lacks as `<hash>.rgba` into the
/// `VERA20K_HD_DUMP_TERRAIN` folder. Nothing when it is unset.
pub(crate) fn dump_tiles<'a>(images: impl IntoIterator<Item = &'a crate::map::theater::TileImage>) {
    dump_rgba(
        images
            .into_iter()
            .map(|tile| (tile.width, tile.height, tile.rgba.as_slice())),
        "rgba",
    );
}

/// Write each `(width, height, rgba)` picture the pack lacks as
/// `<hash>.<extension>` into the `VERA20K_HD_DUMP_TERRAIN` folder: `rgba` for
/// tiles, which the upscaler pads with copies of themselves, `srgba` for
/// overlay and terrain object sprites, padded with their edge colours.
pub(crate) fn dump_rgba<'a>(
    pictures: impl IntoIterator<Item = (u32, u32, &'a [u8])>,
    extension: &str,
) {
    let Some(dump) = std::env::var("VERA20K_HD_DUMP_TERRAIN")
        .ok()
        .map(PathBuf::from)
    else {
        return;
    };
    let pack = pack_dir();
    let _ = std::fs::create_dir_all(&dump);
    let mut written = 0;
    for (width, height, rgba) in pictures {
        if width == 0 || height == 0 || rgba.len() != (width * height * 4) as usize {
            continue;
        }
        let name = format!("{:016x}", tile_hash(width, height, rgba));
        if pack
            .as_ref()
            .is_some_and(|pack| pack.join(format!("{name}.png")).exists())
        {
            continue;
        }
        let path = dump.join(format!("{name}.{extension}"));
        if path.exists() {
            continue;
        }
        let mut bytes = Vec::with_capacity(4 + rgba.len());
        bytes.extend_from_slice(&(width as u16).to_le_bytes());
        bytes.extend_from_slice(&(height as u16).to_le_bytes());
        bytes.extend_from_slice(rgba);
        if std::fs::write(path, bytes).is_ok() {
            written += 1;
        }
    }
    log::info!(
        "HD art terrain dump: {written} new .{extension} in {}",
        dump.display()
    );
}

/// One visible tile or overlay sprite: where its atlas holds it, and where
/// the terrain pass put it in the native composition.
pub(crate) struct HdTerrainPiece {
    pub uv_origin: [f32; 2],
    pub origin: [f32; 2],
    pub size: [f32; 2],
}

/// One HD terrain draw: a tile or sprite placed in the native composition,
/// its place in its atlas, and its place and scale on the HD page.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq)]
pub(crate) struct HdTerrainDraw {
    pub origin: [f32; 2],
    pub size: [f32; 2],
    pub atlas_origin: [u32; 2],
    pub hd_origin: [u32; 2],
    /// HD pixels per native pixel, and padding.
    pub hd_scale: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Params {
    /// Target width, height, target pixels per native pixel, debug view.
    frame: [f32; 4],
    /// x: 1 when the atlas texture is an sRGB view (loads come back linear);
    /// y: 1 when a window mask applies; z: window pixels per frame pixel.
    mode: [f32; 4],
    /// xy: the target pixel of native pixel (0, 0); zw: the window pixel of
    /// frame pixel (0, 0), for the mask.
    place: [f32; 4],
}

/// An atlas's HD key lookup: UV origin → hash, atlas pixel origin, size.
pub(crate) type HdLookup<'a> = &'a dyn Fn([f32; 2]) -> Option<(u64, [u32; 2], [u32; 2])>;

/// Where the native world lands in a target: the frame (origin 0, the wheel
/// zoom's scale) or the window over the Options zoom's copy, with the mask
/// that keeps window pixels whose frame pixel changed after the world went
/// in (`render::picture_zoom::HdMask`, the same rule).
pub(crate) struct TerrainPlacement<'a> {
    pub origin: [f32; 2],
    pub scale: f32,
    pub mask: Option<crate::render::picture_zoom::HdMask<'a>>,
}

/// The terrain HD draw itself, shared by the HD art pack's tiles and
/// sprites ([`HdTerrain`]) and the tile pictures' twins
/// (`render::picture_zoom`).
pub(crate) struct HdTerrainPass {
    layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    shader: wgpu::ShaderModule,
    pipelines: Vec<(wgpu::TextureFormat, wgpu::RenderPipeline)>,
    /// Bound as both mask textures where there is no mask.
    no_mask: wgpu::TextureView,
}

impl HdTerrainPass {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty,
            count: None,
        };
        let tex = wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("HD terrain"),
            entries: &[
                entry(0, tex),
                entry(1, tex),
                entry(2, tex),
                entry(
                    3,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(4, tex),
                entry(5, tex),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("HD terrain"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("HD terrain"),
            source: wgpu::ShaderSource::Wgsl(shader_source().into()),
        });
        let no_mask = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("HD terrain no mask"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        Self {
            layout,
            pipeline_layout,
            shader,
            pipelines: Vec::new(),
            no_mask,
        }
    }

    /// Make the pipeline that draws into `format`, once.
    pub(crate) fn ensure_pipeline(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        if !self.pipelines.iter().any(|(cached, _)| *cached == format) {
            let pipeline = create_pipeline(device, &self.pipeline_layout, &self.shader, format);
            self.pipelines.push((format, pipeline));
        }
    }

    /// Draw `draws` into `target` at `placement`, over `scissor` (target
    /// pixels), reading the native `composition`, the atlas `tiles`
    /// (`linear` when it is an sRGB texture) and the HD page `hd`. Nothing
    /// without a pipeline for the target's format ([`Self::ensure_pipeline`]).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn encode(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        composition: &wgpu::Texture,
        tiles: &wgpu::TextureView,
        linear: bool,
        hd: &wgpu::TextureView,
        placement: &TerrainPlacement<'_>,
        scissor: [u32; 4],
        draws: &[HdTerrainDraw],
        debug: bool,
    ) {
        let size = target.texture().size();
        let Some((_, pipeline)) = self
            .pipelines
            .iter()
            .find(|(format, _)| *format == target.texture().format())
        else {
            return;
        };
        if draws.is_empty() || scissor[2] == 0 || scissor[3] == 0 {
            return;
        }
        let right = (scissor[0] + scissor[2]).min(size.width);
        let bottom = (scissor[1] + scissor[3]).min(size.height);
        if right <= scissor[0] || bottom <= scissor[1] {
            return;
        }
        // Fresh buffers per call: the tiles and the overlays encode in one
        // submission, and a shared buffer written twice would leave both
        // passes reading the second call's data.
        use wgpu::util::DeviceExt;
        let instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("HD terrain draws"),
            contents: bytemuck::cast_slice(draws),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let (now, before, frame_origin, frame_scale, masked) = match &placement.mask {
            Some(mask) => (
                mask.now,
                mask.before,
                mask.frame_origin,
                mask.frame_scale,
                1.0,
            ),
            None => (&self.no_mask, &self.no_mask, [0.0; 2], 1.0, 0.0),
        };
        let params = Params {
            frame: [
                size.width as f32,
                size.height as f32,
                placement.scale,
                if debug { 1.0 } else { 0.0 },
            ],
            mode: [if linear { 1.0 } else { 0.0 }, masked, frame_scale, 0.0],
            place: [
                placement.origin[0],
                placement.origin[1],
                frame_origin[0],
                frame_origin[1],
            ],
        };
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("HD terrain"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let composition_view = composition.create_view(&Default::default());
        let view = |binding, view| wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(view),
        };
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HD terrain"),
            layout: &self.layout,
            entries: &[
                view(0, &composition_view),
                view(1, tiles),
                view(2, hd),
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: uniform.as_entire_binding(),
                },
                view(4, now),
                view(5, before),
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("HD terrain"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_scissor_rect(
            scissor[0],
            scissor[1],
            right - scissor[0],
            bottom - scissor[1],
        );
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_vertex_buffer(0, instances.slice(..));
        pass.draw(0..4, 0..draws.len() as u32);
    }
}

struct Gpu {
    page: wgpu::Texture,
    pass: HdTerrainPass,
}

/// The pack's tiles resident on the GPU, keyed by tile hash.
pub(crate) struct HdTerrain {
    dir: PathBuf,
    /// Tile hash → its HD page origin, or `None` when the pack has none.
    tiles: HashMap<u64, Option<[u32; 2]>>,
    shelf: [u32; 3],
    gpu: Option<Gpu>,
}

impl HdTerrain {
    /// The pack's terrain folder, or `None` when there is none: the pass
    /// then never runs.
    pub(crate) fn open() -> Option<Self> {
        let dir = pack_dir().filter(|dir| dir.is_dir())?;
        log::info!("HD art terrain pack: {}", dir.display());
        Some(Self {
            dir,
            tiles: HashMap::new(),
            shelf: [0, 0, 0],
            gpu: None,
        })
    }

    /// Bring the HD tiles of `pieces` up (at most [`LOADS_PER_FRAME`] new
    /// files a frame) and return this frame's draws, in `pieces` order.
    pub(crate) fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        lookup: HdLookup<'_>,
        pieces: &[HdTerrainPiece],
    ) -> Vec<HdTerrainDraw> {
        self.ensure_gpu(device);
        // A capture shows one frame: it reads everything on screen at once.
        let budget = if super::hd_switch::capture_zoom().is_some() {
            usize::MAX
        } else {
            LOADS_PER_FRAME
        };
        let mut loads = 0;
        let mut draws = Vec::with_capacity(pieces.len());
        let (mut unknown, mut resized) = (0, 0);
        for piece in pieces {
            let Some((hash, atlas_origin, size)) = lookup(piece.uv_origin) else {
                unknown += 1;
                continue;
            };
            if piece.size != [size[0] as f32, size[1] as f32] {
                resized += 1;
                continue;
            }
            let hd_origin = match self.tiles.get(&hash) {
                Some(origin) => *origin,
                None => {
                    if loads >= budget {
                        continue;
                    }
                    loads += 1;
                    let origin = match self.load(queue, hash, size) {
                        Ok(origin) => origin,
                        Err(()) => {
                            log::info!("HD art terrain page full; reloading the tiles on screen");
                            self.tiles.retain(|_, origin| origin.is_none());
                            self.shelf = [0, 0, 0];
                            draws.clear();
                            continue;
                        }
                    };
                    self.tiles.insert(hash, origin);
                    origin
                }
            };
            if let Some(hd_origin) = hd_origin {
                draws.push(HdTerrainDraw {
                    origin: piece.origin,
                    size: piece.size,
                    atlas_origin,
                    hd_origin,
                    hd_scale: [2, 0],
                });
            }
        }
        log::debug!(
            "HD art terrain: {} pieces, {} draws, {unknown} not atlas tiles, {resized} resized",
            pieces.len(),
            draws.len()
        );
        draws
    }

    /// Read `<hash>.png` and place it: `Ok(None)` when the pack lacks it,
    /// `Err` when the page is full.
    fn load(
        &mut self,
        queue: &wgpu::Queue,
        hash: u64,
        [width, height]: [u32; 2],
    ) -> Result<Option<[u32; 2]>, ()> {
        let art = std::fs::read(self.dir.join(format!("{hash:016x}.png")))
            .ok()
            .and_then(|bytes| {
                image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).ok()
            })
            .map(|image| image.into_rgba8());
        let Some(art) = art.filter(|art| art.width() == width * 2 && art.height() == height * 2)
        else {
            log::debug!("HD art terrain: no picture for {hash:016x} ({width}x{height})");
            return Ok(None);
        };
        let [w, h] = [width * 2, height * 2];
        if w > PAGE || h > PAGE {
            return Ok(None);
        }
        let [x, y, row] = &mut self.shelf;
        if *x + w > PAGE {
            *y += *row + 2;
            *x = 0;
            *row = 0;
        }
        if *y + h > PAGE {
            return Err(());
        }
        let origin = [*x, *y];
        *x += w + 2;
        *row = (*row).max(h);
        let gpu = self.gpu.as_ref().expect("HD terrain GPU resources");
        super::atlas_growth::write_texels(queue, &gpu.page, origin, [w, h], 4, &art.into_raw());
        Ok(Some(origin))
    }

    fn ensure_gpu(&mut self, device: &wgpu::Device) {
        if self.gpu.is_some() {
            return;
        }
        let page = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("HD terrain page"),
            size: wgpu::Extent3d {
                width: PAGE,
                height: PAGE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.gpu = Some(Gpu {
            page,
            pass: HdTerrainPass::new(device),
        });
    }

    /// Draw `draws` into `target` (the zoomed frame, `scale` frame pixels
    /// per native pixel) over `scissor`, reading the native `composition`
    /// and the atlas `tiles` (`linear` when it is an sRGB texture).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn encode(
        &mut self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::Texture,
        composition: &wgpu::Texture,
        tiles: &wgpu::TextureView,
        linear: bool,
        scale: f32,
        scissor: [u32; 4],
        draws: &[HdTerrainDraw],
        debug: bool,
    ) {
        let Some(gpu) = self.gpu.as_mut() else {
            return;
        };
        gpu.pass.ensure_pipeline(device, target.format());
        let page_view = gpu.page.create_view(&Default::default());
        gpu.pass.encode(
            device,
            encoder,
            &target.create_view(&Default::default()),
            composition,
            tiles,
            linear,
            &page_view,
            &TerrainPlacement {
                origin: [0.0; 2],
                scale,
                mask: None,
            },
            scissor,
            draws,
            debug,
        );
    }
}

fn shader_source() -> String {
    format!(
        "{}\n{}",
        super::webgl::srgb_wgsl(),
        include_str!("hd_terrain.wgsl")
    )
}

fn create_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let attributes = wgpu::vertex_attr_array![
        0 => Float32x2, 1 => Float32x2, 2 => Uint32x2, 3 => Uint32x2, 4 => Uint32x2
    ];
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("HD terrain"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<HdTerrainDraw>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &attributes,
            })],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg(all(test, tests_render_a_r))]
mod tests {
    use super::*;

    #[test]
    fn hd_terrain_shader_parses_and_validates() {
        use wgpu::naga;
        let module =
            naga::front::wgsl::parse_str(&shader_source()).expect("HD terrain shader parses");
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::empty(),
        )
        .validate(&module)
        .expect("HD terrain shader validates");
    }
}
