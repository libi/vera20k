//! VERA-only, experimental picture art (`Picture=`,
//! `docs/picture-model-art.md`): a declared picture's frames as the SHP
//! blitters' RGBA source pages.
//!
//! Each frame is shrunk to game pixels (`PictureSize=`), cropped to its drawn
//! pixels like a stored SHP frame, and given an index plane the blitters read
//! as palette indices: 0 for a hole, [`LIT_INDEX`] for an ordinary pixel and
//! [`UNLIT_INDEX`] for one light leaves alone (the mask's green). House-colour
//! pixels (the mask's red) take the owner's ramp by brightness before upload.
//! The SHP blitters, their shaders and their Z, shadow and light rules are
//! the ones every SHP draws through; nothing here changes them.
//!
//! Art above 1x keeps only its game pixels here, decoded one frame at a time,
//! and where its files are ([`HdFiles`]). The zoomed draw asks for a frame at
//! the picture's own resolution when it needs one ([`hd_sprite`],
//! `render::picture_hd`); the interface strips, which are small, keep theirs
//! ([`HdFrames`]).

use std::sync::Arc;

use crate::assets::mix_archive::SharedBytes;
use crate::assets::pal_file::Color;
use crate::assets::picture_file::{PictureError, PictureFrameReader, PictureFrames};

/// A house colour ramp's entries, palette indices 16–31
/// (`HouseColorRamps::ramp`).
pub(crate) const RAMP_SIZE: usize = 16;

/// The index-plane value of an ordinary drawn pixel: any index 1..=239 is lit
/// the same way (`native_palette_word`).
pub(crate) const LIT_INDEX: u8 = 1;
/// The index-plane value of a pixel light leaves as it is (indices 240–254,
/// `palette_light.wgsl`).
pub(crate) const UNLIT_INDEX: u8 = 240;
/// The largest whole multiple a picture may have of its `PictureSize=`.
pub(crate) const PICTURE_SCALE_MAX: u32 = 8;

/// What a mask pixel marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaskClass {
    Plain,
    House,
    Unlit,
}

/// A picture's frames at game pixels: straight RGBA with binary alpha, the
/// mask's class of each pixel, on the `PictureSize=` canvas.
#[derive(Debug, Clone)]
pub(crate) struct GamePictureFrames {
    pub canvas: [u32; 2],
    frames: Vec<GameFrame>,
    /// The picture's pixels per game pixel on each axis.
    scale: u32,
    /// VERA-only, experimental: with `scale` above 1, whether each frame has
    /// a drawn house-colour pixel at the picture's own resolution, so its HD
    /// twin differs by house.
    hd_house: Vec<bool>,
    /// VERA-only, experimental: with `scale` above 1, the files its frames
    /// are decoded from again at their own resolution; None for 1x art and
    /// for a picture shrunk from frames in memory.
    hd_files: Option<Arc<HdFiles>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameFrame {
    rgba: Vec<u8>,
    classes: Vec<MaskClass>,
}

/// One frame ready for a page: the drawn rectangle of the canvas, its RGBA
/// and its index plane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PictureSprite {
    /// The drawn rectangle's place on the canvas, as a stored SHP frame's.
    pub origin: [u32; 2],
    pub size: [u32; 2],
    pub rgba: Vec<u8>,
    pub indices: Vec<u8>,
}

impl PictureSprite {
    /// Whether an SHP of this frame stores it RLE-compressed (format 3,
    /// format bit 0x02, the "extended" walker) rather than raw (format 1):
    /// the shape tools keep the smaller, and an RLE row is its 2-byte length,
    /// one byte a drawn pixel and two (`0x00`, count) a run of up to 255
    /// holes. This picks each retail frame's format exactly (GI, ROCK,
    /// FIRE01, EXPLOSML and NGPOWR_A checked frame by frame).
    pub(crate) fn stored_as_rle(&self) -> bool {
        let [width, height] = self.size.map(|side| side as usize);
        let rle: usize = self
            .indices
            .chunks_exact(width.max(1))
            .map(|row| {
                let (mut bytes, mut run) = (2, 0);
                for &index in row {
                    if index == 0 {
                        run += 1;
                        if run == 255 {
                            (bytes, run) = (bytes + 2, 0);
                        }
                    } else {
                        if run > 0 {
                            (bytes, run) = (bytes + 2, 0);
                        }
                        bytes += 1;
                    }
                }
                bytes + if run > 0 { 2 } else { 0 }
            })
            .sum();
        rle < width * height
    }
}

/// VERA-only, experimental: a sprite's rectangle at the picture's own
/// resolution, `scale` picture pixels per game pixel. Its RGBA is
/// premultiplied (holes are 0, so filtering never darkens an edge) and its
/// index plane holds the sprite's index-plane values per picture pixel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HdSprite {
    pub scale: u32,
    pub size: [u32; 2],
    pub rgba: Vec<u8>,
    pub indices: Vec<u8>,
}

/// VERA-only, experimental: where an atlas picture above 1x is decoded from
/// at its own resolution, the picture and its mask as the archive or folder
/// holds them (a share of their storage, never decoded here).
pub(crate) struct HdFiles {
    /// The picture as declared, which names it in a log line.
    pub source: PictureSource,
    /// The file its 1x frames were decoded from, after the theater lookup
    /// (`ArtRegistry::picture_candidates`): `picture` is its bytes, the very
    /// share the 1x decode read, so the twin never comes from another file.
    pub file: String,
    picture: SharedBytes,
    mask: Option<SharedBytes>,
    pub scale: u32,
}

impl std::fmt::Debug for HdFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HdFiles")
            .field("source", &self.source)
            .field("file", &self.file)
            .field("scale", &self.scale)
            .finish_non_exhaustive()
    }
}

impl HdFiles {
    /// Decode the picture and its mask one frame at a time up to frame
    /// `last`, handing `each` every frame's index, its RGBA and its mask's
    /// RGBA (`width` picture pixels a row). Holds one frame of each.
    pub(crate) fn each_frame(
        &self,
        last: usize,
        mut each: impl FnMut(usize, &[u8], Option<&[u8]>, u32),
    ) -> Result<(), PictureArtError> {
        let mut picture = PictureFrameReader::new(&self.picture)?;
        let mut mask = self
            .mask
            .as_deref()
            .map(PictureFrameReader::new)
            .transpose()?;
        let width = picture.size().0;
        for index in 0..=last {
            let Some(rgba) = picture.next_frame()? else {
                break;
            };
            let mask = match mask.as_mut() {
                Some(mask) => mask.next_frame()?,
                None => None,
            };
            each(index, rgba, mask, width);
        }
        Ok(())
    }
}

/// VERA-only, experimental: a picture's frames at its own resolution and its
/// mask, kept whole, for the interface strips and the placement preview,
/// which upload every frame's HD twin at once
/// (`CenteredShapeStrip::from_picture_on_device`).
pub(crate) struct HdFrames {
    picture: PictureFrames,
    mask: Option<PictureFrames>,
    scale: u32,
}

impl HdFrames {
    /// `picture` and `mask` at `scale` picture pixels per game pixel; None
    /// for 1x art, which has no HD twin.
    pub(crate) fn new(
        picture: PictureFrames,
        mask: Option<PictureFrames>,
        scale: u32,
    ) -> Option<Self> {
        (scale > 1).then_some(Self {
            picture,
            mask,
            scale,
        })
    }

    pub(crate) fn scale(&self) -> u32 {
        self.scale
    }

    /// Frame `frame`'s `sprite` rectangle at the picture's own resolution
    /// ([`hd_sprite`]); None for a frame past the end.
    pub(crate) fn sprite(
        &self,
        frame: usize,
        sprite: &PictureSprite,
        ramp: Option<&[Color; RAMP_SIZE]>,
    ) -> Option<HdSprite> {
        let rgba = self.picture.frames.get(frame)?;
        let mask = self
            .mask
            .as_ref()
            .and_then(|mask| mask.frames.get(frame))
            .map(Vec::as_slice);
        Some(hd_sprite(
            rgba,
            mask,
            self.picture.width,
            self.scale,
            sprite.origin,
            sprite.size,
            ramp,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PictureArtError {
    #[error(transparent)]
    Decode(#[from] PictureError),
    #[error(
        "{picture:?} is not a whole multiple (1 to {PICTURE_SCALE_MAX}) of PictureSize={canvas:?}"
    )]
    Scale { picture: [u32; 2], canvas: [u32; 2] },
    #[error(
        "the mask is {mask:?} with {mask_frames} frames, the picture {picture:?} with {frames}"
    )]
    MaskShape {
        mask: [u32; 2],
        mask_frames: usize,
        picture: [u32; 2],
        frames: usize,
    },
    #[error(
        "{0} bytes of frames at game pixels is more than the {GAME_FRAMES_BYTES_MAX} a picture may hold"
    )]
    GameTooLarge(u64),
}

/// The most RGBA bytes one picture's frames may take at game pixels, all
/// frames together, which the atlas keeps while it builds (each frame is
/// read and shrunk alone, so its own resolution is bounded per frame,
/// `PICTURE_FRAME_BYTES_MAX`).
pub(crate) const GAME_FRAMES_BYTES_MAX: u64 = crate::assets::picture_file::PICTURE_BYTES_MAX;

/// The frames of one picture shrunk as they come ([`GamePictureFrames::shrink`]).
struct Shrinker {
    width: u32,
    canvas: [u32; 2],
    scale: u32,
    frames: Vec<GameFrame>,
    hd_house: Vec<bool>,
}

impl Shrinker {
    /// The shrinker of a `size` picture onto `canvas`, or the scale error.
    fn new(
        size: [u32; 2],
        canvas: Option<[u32; 2]>,
        frames: usize,
    ) -> Result<Self, PictureArtError> {
        let canvas = canvas.unwrap_or(size);
        let scale = size[0] / canvas[0].max(1);
        if scale == 0 || scale > PICTURE_SCALE_MAX || size != [canvas[0] * scale, canvas[1] * scale]
        {
            return Err(PictureArtError::Scale {
                picture: size,
                canvas,
            });
        }
        let game_bytes = u64::from(canvas[0]) * u64::from(canvas[1]) * 4 * frames as u64;
        if game_bytes > GAME_FRAMES_BYTES_MAX {
            return Err(PictureArtError::GameTooLarge(game_bytes));
        }
        Ok(Self {
            width: size[0],
            canvas,
            scale,
            frames: Vec::with_capacity(frames),
            hd_house: Vec::with_capacity(if scale > 1 { frames } else { 0 }),
        })
    }

    fn push(&mut self, rgba: &[u8], mask: Option<&[u8]>) {
        let (frame, house) = shrink_frame(rgba, mask, self.width, self.canvas, self.scale);
        self.frames.push(frame);
        if self.scale > 1 {
            self.hd_house.push(house);
        }
    }

    fn finish(self) -> GamePictureFrames {
        GamePictureFrames {
            canvas: self.canvas,
            frames: self.frames,
            scale: self.scale,
            hd_house: self.hd_house,
            hd_files: None,
        }
    }
}

impl GamePictureFrames {
    /// Shrink `picture`, and its `mask` when it has one, to `canvas` game
    /// pixels. A game pixel is drawn when at least half the picture pixels it
    /// covers are; its colour is their coverage-weighted mean, and it takes
    /// the class most of its covered pixels have. Without `canvas` the
    /// picture is 1x art.
    pub(crate) fn shrink(
        picture: &PictureFrames,
        mask: Option<&PictureFrames>,
        canvas: Option<[u32; 2]>,
    ) -> Result<Self, PictureArtError> {
        let size = [picture.width, picture.height];
        let mut shrinker = Shrinker::new(size, canvas, picture.frames.len())?;
        if let Some(mask) = mask
            && ([mask.width, mask.height] != size || mask.frames.len() != picture.frames.len())
        {
            return Err(PictureArtError::MaskShape {
                mask: [mask.width, mask.height],
                mask_frames: mask.frames.len(),
                picture: size,
                frames: picture.frames.len(),
            });
        }
        for (index, rgba) in picture.frames.iter().enumerate() {
            shrinker.push(rgba, mask.map(|mask| mask.frames[index].as_slice()));
        }
        Ok(shrinker.finish())
    }

    /// [`Self::shrink`] of the PNG or APNG `picture` and its `mask`, decoded
    /// and shrunk one frame at a time, so no more than one frame of each is
    /// held at the picture's own resolution.
    pub(crate) fn shrink_png(
        picture: &[u8],
        mask: Option<&[u8]>,
        canvas: Option<[u32; 2]>,
    ) -> Result<Self, PictureArtError> {
        let mut picture = PictureFrameReader::new(picture)?;
        let mut mask = mask.map(PictureFrameReader::new).transpose()?;
        let (width, height) = picture.size();
        let count = picture.frame_count();
        let mut shrinker = Shrinker::new([width, height], canvas, count as usize)?;
        if let Some(mask) = &mask
            && (mask.size() != (width, height) || mask.frame_count() != count)
        {
            return Err(PictureArtError::MaskShape {
                mask: [mask.size().0, mask.size().1],
                mask_frames: mask.frame_count() as usize,
                picture: [width, height],
                frames: count as usize,
            });
        }
        while let Some(rgba) = picture.next_frame()? {
            let mask = match mask.as_mut() {
                Some(mask) => mask.next_frame()?,
                None => None,
            };
            shrinker.push(rgba, mask);
        }
        Ok(shrinker.finish())
    }

    pub(crate) fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Frame `frame` cropped to its drawn pixels, house-colour pixels in
    /// `ramp` (none leaves them their own colour). None for a frame past the
    /// end or with nothing drawn, as an SHP's empty frame.
    pub(crate) fn sprite(
        &self,
        frame: usize,
        ramp: Option<&[Color; RAMP_SIZE]>,
    ) -> Option<PictureSprite> {
        let source = self.frames.get(frame)?;
        let width = self.canvas[0];
        let ([left, top], size) = self.drawn_rect(frame)?;
        let (right, bottom) = (left + size[0], top + size[1]);
        let mut rgba = Vec::with_capacity((size[0] * size[1] * 4) as usize);
        let mut indices = Vec::with_capacity((size[0] * size[1]) as usize);
        for y in top..bottom {
            for x in left..right {
                let at = (y * width + x) as usize;
                let mut pixel: [u8; 4] = source.rgba[at * 4..at * 4 + 4].try_into().unwrap();
                let index = match (pixel[3], source.classes[at]) {
                    (0, _) => 0,
                    (_, MaskClass::Unlit) => UNLIT_INDEX,
                    (_, MaskClass::House) => {
                        if let Some(ramp) = ramp {
                            let color = house_ramp_color(pixel, ramp);
                            pixel = [color.r, color.g, color.b, 255];
                        }
                        LIT_INDEX
                    }
                    (_, MaskClass::Plain) => LIT_INDEX,
                };
                rgba.extend_from_slice(&pixel);
                indices.push(index);
            }
        }
        Some(PictureSprite {
            origin: [left, top],
            size,
            rgba,
            indices,
        })
    }

    /// VERA-only, experimental: what the zoomed draw needs to make frame
    /// `frame`'s `sprite` at the picture's own resolution later, house colour
    /// in `house`'s ramp; None for 1x art and a picture with no files. A
    /// frame without a drawn house-colour pixel takes no house, so every
    /// house shares one twin of it.
    pub(crate) fn hd_twin(
        &self,
        frame: usize,
        sprite: &PictureSprite,
        house: Option<(
            crate::rules::house_colors::HouseColorIndex,
            &[Color; RAMP_SIZE],
        )>,
    ) -> Option<HdTwin> {
        let files = self.hd_files.as_ref()?;
        let house = house.filter(|_| self.hd_house.get(frame).copied().unwrap_or(true));
        Some(HdTwin {
            files: Arc::clone(files),
            frame: u32::try_from(frame).ok()?,
            origin: sprite.origin,
            size: sprite.size,
            house: house.map(|(index, ramp)| (index, *ramp)),
        })
    }

    /// Frame `frame`'s drawn rectangle on the canvas, origin and size, as a
    /// stored SHP frame's; None for a frame past the end or with nothing
    /// drawn.
    fn drawn_rect(&self, frame: usize) -> Option<([u32; 2], [u32; 2])> {
        let source = self.frames.get(frame)?;
        let width = self.canvas[0];
        let drawn = |x: u32, y: u32| source.rgba[((y * width + x) * 4 + 3) as usize] != 0;
        let (mut left, mut top, mut right, mut bottom) = (u32::MAX, u32::MAX, 0, 0);
        for y in 0..self.canvas[1] {
            for x in 0..width {
                if drawn(x, y) {
                    (left, top) = (left.min(x), top.min(y));
                    (right, bottom) = (right.max(x + 1), bottom.max(y + 1));
                }
            }
        }
        (left < right).then(|| ([left, top], [right - left, bottom - top]))
    }

    /// Frame `frame`'s rectangle `[x, y, w, h]` as `SHPStruct::GetFrameBounds`
    /// (`0x0069E7E0`) reads a stored SHP frame's: all zero for an empty frame
    /// or one past the end.
    fn frame_bounds(&self, frame: usize) -> [i32; 4] {
        self.drawn_rect(frame).map_or([0; 4], |(origin, size)| {
            let word = |value: u32| i32::try_from(value).unwrap_or(i32::MAX);
            [
                word(origin[0]),
                word(origin[1]),
                word(size[0]),
                word(size[1]),
            ]
        })
    }

    /// Whether any drawn pixel takes the owner's colour, so the frames
    /// differ by house.
    #[cfg(test)]
    pub(crate) fn has_house_color(&self) -> bool {
        self.frames.iter().any(|frame| {
            frame
                .classes
                .iter()
                .zip(frame.rgba.chunks_exact(4))
                .any(|(class, pixel)| *class == MaskClass::House && pixel[3] != 0)
        })
    }

    /// Frame `frame` without house colour, placed as a stored SHP frame
    /// relative to its draw point (CC_Draw_Shape `0x004AED70`: the canvas
    /// half, truncated, then the frame's origin; `stored_frame_offset`).
    /// None when it draws nothing.
    pub(crate) fn stored_frame(&self, frame: usize) -> Option<StoredPictureFrame> {
        let sprite = self.sprite(frame, None)?;
        let half = self
            .canvas
            .map(|side| i32::try_from(side / 2).unwrap_or(i32::MAX));
        let origin = sprite
            .origin
            .map(|side| i32::try_from(side).unwrap_or(i32::MAX));
        Some(StoredPictureFrame {
            offset: [(origin[0] - half[0]) as f32, (origin[1] - half[1]) as f32],
            draw_point: [half[0] - origin[0], half[1] - origin[1]],
            sprite,
        })
    }
}

/// VERA-only, experimental: an atlas sprite's HD twin as a request, the
/// light record an atlas key above 1x keeps instead of its pixels
/// (`render::picture_hd`).
#[derive(Debug, Clone)]
pub(crate) struct HdTwin {
    pub files: Arc<HdFiles>,
    pub frame: u32,
    /// The sprite's rectangle on the canvas, in game pixels.
    pub origin: [u32; 2],
    pub size: [u32; 2],
    /// The house whose ramp colours its house-colour pixels; None leaves
    /// them their own colour, as for a frame that has none.
    pub house: Option<(
        crate::rules::house_colors::HouseColorIndex,
        [Color; RAMP_SIZE],
    )>,
}

/// Frame `rgba` (`picture_width` picture pixels a row, with its `mask` when
/// it has one) at its own resolution over the game-pixel rectangle at
/// `origin`, `size`, `scale` picture pixels per game pixel. A picture pixel
/// is drawn when at least half covered, as `shrink` counts it; house-colour
/// pixels take `ramp` when given.
pub(crate) fn hd_sprite(
    rgba: &[u8],
    mask: Option<&[u8]>,
    picture_width: u32,
    scale: u32,
    origin: [u32; 2],
    size: [u32; 2],
    ramp: Option<&[Color; RAMP_SIZE]>,
) -> HdSprite {
    let hd_size = size.map(|side| side * scale);
    let pixels = (hd_size[0] * hd_size[1]) as usize;
    let (mut out, mut indices) = (Vec::with_capacity(pixels * 4), Vec::with_capacity(pixels));
    for y in origin[1] * scale..origin[1] * scale + hd_size[1] {
        for x in origin[0] * scale..origin[0] * scale + hd_size[0] {
            let at = ((y * picture_width + x) * 4) as usize;
            let pixel = &rgba[at..at + 4];
            if u32::from(pixel[3]) * 2 < 255 {
                out.extend_from_slice(&[0; 4]);
                indices.push(0);
                continue;
            }
            let class = mask.map_or(MaskClass::Plain, |mask| mask_class(&mask[at..at + 4]));
            let (color, index) = match (class, ramp) {
                (MaskClass::Unlit, _) => ([pixel[0], pixel[1], pixel[2]], UNLIT_INDEX),
                (MaskClass::House, Some(ramp)) => {
                    let color = house_ramp_color([pixel[0], pixel[1], pixel[2], 255], ramp);
                    ([color.r, color.g, color.b], LIT_INDEX)
                }
                _ => ([pixel[0], pixel[1], pixel[2]], LIT_INDEX),
            };
            out.extend_from_slice(&[color[0], color[1], color[2], 255]);
            indices.push(index);
        }
    }
    HdSprite {
        scale,
        size: hd_size,
        rgba: out,
        indices,
    }
}

/// A frame placed as a stored SHP frame ([`GamePictureFrames::stored_frame`]).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredPictureFrame {
    pub sprite: PictureSprite,
    /// The sprite's top-left corner from the draw point.
    pub offset: [f32; 2],
    /// The draw point's place in the sprite.
    pub draw_point: [i32; 2],
}

/// One frame of `picture_width`-wide RGBA shrunk by `scale` onto `canvas`,
/// and whether any drawn picture pixel of it is house colour.
fn shrink_frame(
    rgba: &[u8],
    mask: Option<&[u8]>,
    picture_width: u32,
    canvas: [u32; 2],
    scale: u32,
) -> (GameFrame, bool) {
    let pixels = (canvas[0] * canvas[1]) as usize;
    let mut out = GameFrame {
        rgba: Vec::with_capacity(pixels * 4),
        classes: Vec::with_capacity(pixels),
    };
    let mut house = false;
    let area = scale * scale;
    for y in 0..canvas[1] {
        for x in 0..canvas[0] {
            let (mut covered, mut sum) = (0u32, [0u32; 3]);
            let mut votes = [0u32; 3];
            for dy in 0..scale {
                for dx in 0..scale {
                    let at = (((y * scale + dy) * picture_width + x * scale + dx) * 4) as usize;
                    let alpha = u32::from(rgba[at + 3]);
                    if alpha * 2 < 255 {
                        continue;
                    }
                    covered += 1;
                    for channel in 0..3 {
                        sum[channel] += u32::from(rgba[at + channel]);
                    }
                    let class = mask.map_or(MaskClass::Plain, |mask| mask_class(&mask[at..at + 4]));
                    votes[class as usize] += 1;
                }
            }
            house |= votes[MaskClass::House as usize] > 0;
            if covered * 2 < area {
                out.rgba.extend_from_slice(&[0; 4]);
                out.classes.push(MaskClass::Plain);
                continue;
            }
            let mean = |channel: usize| ((sum[channel] + covered / 2) / covered) as u8;
            out.rgba
                .extend_from_slice(&[mean(0), mean(1), mean(2), 255]);
            let class = [MaskClass::Plain, MaskClass::House, MaskClass::Unlit]
                .into_iter()
                .max_by_key(|class| (votes[*class as usize], *class == MaskClass::Plain))
                .unwrap();
            out.classes.push(class);
        }
    }
    (out, house)
}

/// A mask pixel's class: red at 128 or more marks house colour, else green at
/// 128 or more marks an unlit pixel (`docs/picture-model-art.md`).
fn mask_class(pixel: &[u8]) -> MaskClass {
    if pixel[0] >= 128 {
        MaskClass::House
    } else if pixel[1] >= 128 {
        MaskClass::Unlit
    } else {
        MaskClass::Plain
    }
}

/// A house-colour pixel's colour on the owner's `ramp`: its brightness, the
/// largest of red, green and blue, picks the step, 255 the lightest (ramp
/// entry 0, palette index 16) and every 17 levels darker one step down.
fn house_ramp_color(pixel: [u8; 4], ramp: &[Color; RAMP_SIZE]) -> Color {
    let brightness = pixel[0].max(pixel[1]).max(pixel[2]);
    let step = usize::from((255 - brightness) / 17).min(RAMP_SIZE - 1);
    ramp[step]
}

/// One atlas key drawn from a picture: its sprite, placed as
/// `render_shp_frame` places an SHP frame (CC_Draw_Shape `0x004AED70`
/// centres the canvas and adds the frame's origin, and the art's
/// XDrawOffset/YDrawOffset).
pub(crate) struct PlacedPicture {
    pub key: crate::render::sprite_atlas::ShpSpriteKey,
    pub sprite: PictureSprite,
    pub offset: [f32; 2],
    pub canvas_rect: [f32; 4],
    /// The SHP frame format bit (`ShpSpriteEntry::extended`) the frame's SHP
    /// would carry ([`PictureSprite::stored_as_rle`]): it decides whether the
    /// extended walker consumes BUILDNGZ and how the Z walk is seeded.
    pub extended: bool,
    /// VERA-only, experimental: for art above 1x, what makes its HD twin
    /// when the zoomed draw asks for it (`render::picture_hd`).
    pub hd: Option<HdTwin>,
}

/// Where an atlas key's frames are drawn from: the art section that declares
/// `Picture=`, which gives the theater substitution and the log name, and the
/// picture, the section's own `Picture=` or one a building's `Buildup=` or
/// `BibShape=` names beside it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PictureSource {
    pub section: String,
    pub decl: crate::rules::art_data::PictureDecl,
}

impl PictureSource {
    /// The picture `name` names beside this section's `Picture=`: `NAME.png`,
    /// on the same `PictureSize=` canvas (a building's `Buildup=` and
    /// `BibShape=`, `docs/picture-model-art.md`).
    pub(crate) fn named(&self, name: &str) -> Self {
        Self {
            section: self.section.clone(),
            decl: crate::rules::art_data::PictureDecl {
                file: format!("{name}.png"),
                size: self.decl.size,
            },
        }
    }
}

/// The pictures one atlas build draws, each decoded and shrunk once, by
/// [`PictureSource`]. A picture that is missing or does not fit its canvas is
/// logged once and draws nothing.
#[derive(Default)]
pub(crate) struct PictureSources {
    decoded: std::collections::HashMap<PictureSource, Option<GamePictureFrames>>,
}

impl PictureSources {
    /// `source`'s picture at game pixels, decoding it on first use.
    pub(crate) fn frames(
        &mut self,
        source: &PictureSource,
        art: &crate::rules::art_data::ArtRegistry,
        asset_manager: &crate::assets::asset_manager::AssetManager,
        theater_name: &str,
    ) -> Option<&GamePictureFrames> {
        self.decoded
            .entry(source.clone())
            .or_insert_with(|| {
                decode_source(source, art, asset_manager, theater_name)
                    .and_then(|(file, decoded)| report_decoded(source, &file, decoded))
            })
            .as_ref()
    }

    /// Decode each of `sources` not decoded yet, side by side on the worker
    /// pool, so the atlas build that follows finds them all decoded. A map
    /// whose art declares many pictures otherwise decodes and shrinks them one
    /// by one on the loading thread. Logs and art problems are told in the
    /// order of `sources`, as [`Self::frames`] would tell them.
    pub(crate) fn prefetch<'s>(
        &mut self,
        sources: impl IntoIterator<Item = &'s PictureSource>,
        art: &crate::rules::art_data::ArtRegistry,
        asset_manager: &crate::assets::asset_manager::AssetManager,
        theater_name: &str,
    ) {
        let mut seen = std::collections::HashSet::new();
        let todo: Vec<&PictureSource> = sources
            .into_iter()
            .filter(|source| !self.decoded.contains_key(*source) && seen.insert(*source))
            .collect();
        let decoded = crate::sim::parallel::map(&todo, |source| {
            decode_source(source, art, asset_manager, theater_name)
        });
        for (source, decoded) in todo.into_iter().zip(decoded) {
            let frames = decoded.and_then(|(file, decoded)| report_decoded(source, &file, decoded));
            self.decoded.insert(source.clone(), frames);
        }
    }

    /// `key`'s frame of `source`, placed at the art's `draw_offsets`, house
    /// colour in `ramp`; None when it draws nothing, or when
    /// [`Self::frames`] or [`Self::prefetch`] has not decoded `source`.
    /// `select_frame` picks the frame drawn from the picture's frame count,
    /// as `render_shp_frame` picks an SHP's. Reads only, so keys can be
    /// placed side by side.
    pub(crate) fn place_decoded(
        &self,
        key: &crate::render::sprite_atlas::ShpSpriteKey,
        source: &PictureSource,
        draw_offsets: (i32, i32),
        select_frame: impl FnOnce(usize) -> Option<usize>,
        ramp: Option<&[Color; RAMP_SIZE]>,
    ) -> Option<PlacedPicture> {
        let frames = self.decoded_frames(source)?;
        let frame = select_frame(frames.frame_count())?;
        let sprite = frames.sprite(frame, ramp)?;
        let hd = frames.hd_twin(frame, &sprite, ramp.map(|ramp| (key.house_color, ramp)));
        let [width, height] = frames.canvas;
        // -ShapeWidth/2, truncated, as `render_shp_frame`.
        let left = -((width / 2) as f32) + draw_offsets.0 as f32;
        let top = -((height / 2) as f32) + draw_offsets.1 as f32;
        Some(PlacedPicture {
            key: key.clone(),
            offset: [
                left + sprite.origin[0] as f32,
                top + sprite.origin[1] as f32,
            ],
            canvas_rect: [left, top, width as f32, height as f32],
            extended: sprite.stored_as_rle(),
            sprite,
            hd,
        })
    }

    /// VERA-only, experimental: `source`'s picture at its own resolution,
    /// every frame and its mask, for a strip that uploads its HD twin whole
    /// (the placement preview); None for 1x art or a picture
    /// [`Self::frames`] cannot draw.
    #[cfg_attr(not(feature = "phobos"), allow(dead_code))]
    pub(crate) fn hd_frames(
        &mut self,
        source: &PictureSource,
        art: &crate::rules::art_data::ArtRegistry,
        asset_manager: &crate::assets::asset_manager::AssetManager,
        theater_name: &str,
    ) -> Option<HdFrames> {
        let files = self
            .frames(source, art, asset_manager, theater_name)?
            .hd_files
            .clone()?;
        let picture = PictureFrames::decode(&files.picture).ok()?;
        let mask = files
            .mask
            .as_deref()
            .map(PictureFrames::decode)
            .transpose()
            .ok()?;
        HdFrames::new(picture, mask, files.scale)
    }

    /// `BuildingTypeClass::GetRenderDimensions` (`0x00465570`) of a building
    /// drawn from pictures: [`building_render_bounds`] of its `body`,
    /// `buildup` and `bib` pictures. A buildup or bib picture that cannot be
    /// drawn is left out, as an SHP the theater lacks; a body that cannot be
    /// drawn gives the empty rectangle, as a missing body SHP does.
    pub(crate) fn building_bounds(
        &mut self,
        art: &crate::rules::art_data::ArtRegistry,
        asset_manager: &crate::assets::asset_manager::AssetManager,
        theater_name: &str,
        body: &PictureSource,
        buildup: Option<&PictureSource>,
        bib: Option<&PictureSource>,
    ) -> [i32; 4] {
        for source in [Some(body), buildup, bib].into_iter().flatten() {
            self.frames(source, art, asset_manager, theater_name);
        }
        let Some(body) = self.decoded_frames(body) else {
            return [0; 4];
        };
        building_render_bounds(
            body,
            buildup.and_then(|source| self.decoded_frames(source)),
            bib.and_then(|source| self.decoded_frames(source)),
        )
    }

    /// Whether [`Self::frames`] or [`Self::prefetch`] decoded `source` and it
    /// can be drawn.
    pub(crate) fn is_decoded(&self, source: &PictureSource) -> bool {
        self.decoded_frames(source).is_some()
    }

    /// `source`'s picture if [`Self::frames`] decoded it.
    fn decoded_frames(&self, source: &PictureSource) -> Option<&GamePictureFrames> {
        self.decoded.get(source).and_then(Option::as_ref)
    }
}

/// `BuildingTypeClass::GetRenderDimensions` (`0x00465570`) over pictures, as
/// `sprite_atlas::building_type_render_bounds` folds an SHP's frame
/// rectangles: every body frame, every buildup frame, then bib frame 0, in
/// that order, centred by the body's canvas only.
pub(crate) fn building_render_bounds(
    body: &GamePictureFrames,
    buildup: Option<&GamePictureFrames>,
    bib: Option<&GamePictureFrames>,
) -> [i32; 4] {
    let mut accumulated = None;
    for frames in [Some(body), buildup].into_iter().flatten() {
        for frame in 0..frames.frame_count() {
            crate::util::rect::union_native_draw_bounds(
                &mut accumulated,
                frames.frame_bounds(frame),
            );
        }
    }
    if let Some(bib) = bib {
        crate::util::rect::union_native_draw_bounds(&mut accumulated, bib.frame_bounds(0));
    }
    let mut bounds = accumulated.unwrap_or([0; 4]);
    let canvas = body
        .canvas
        .map(|side| i32::try_from(side).unwrap_or(i32::MAX));
    bounds[0] -= canvas[0] / 2;
    bounds[1] -= canvas[1] / 2;
    bounds
}

/// The file `source` loads in `theater_name`, and its picture at game pixels
/// ([`shrink_picture`]); None when no candidate file is there. Pure: what
/// it found is told by [`report_decoded`].
fn decode_source(
    source: &PictureSource,
    art: &crate::rules::art_data::ArtRegistry,
    asset_manager: &crate::assets::asset_manager::AssetManager,
    theater_name: &str,
) -> Option<(String, Result<GamePictureFrames, PictureArtError>)> {
    let (file, bytes) = art
        .picture_candidates(&source.section, &source.decl, theater_name)
        .into_iter()
        .find_map(|file| asset_manager.get_shared(&file).map(|bytes| (file, bytes)))?;
    let mask = asset_manager.get_shared(&mask_name(&file));
    let decoded = GamePictureFrames::decode_files(source, &file, bytes, mask);
    Some((file, decoded))
}

impl GamePictureFrames {
    /// `source`'s `picture` file, named `file`, and its `mask` at game pixels, shrunk one
    /// frame at a time ([`Self::shrink_png`]). Art above 1x keeps where its
    /// frames are ([`HdFiles`]), for the zoomed draw.
    pub(crate) fn decode_files(
        source: &PictureSource,
        file: &str,
        picture: SharedBytes,
        mask: Option<SharedBytes>,
    ) -> Result<Self, PictureArtError> {
        let frames = shrink_png_with_mask(&picture, mask.as_deref(), &source.decl)?;
        let hd_files = (frames.scale > 1).then(|| {
            Arc::new(HdFiles {
                source: source.clone(),
                file: file.to_owned(),
                picture,
                mask,
                scale: frames.scale,
            })
        });
        Ok(Self { hd_files, ..frames })
    }
}

/// `NAME.mask.png` beside the picture `file`.
fn mask_name(file: &str) -> String {
    format!(
        "{}.mask.png",
        file.rsplit_once('.').map_or(file, |(stem, _)| stem)
    )
}

/// The picture `bytes` and its `mask` shrunk to the game pixels `decl`
/// gives, one frame at a time ([`GamePictureFrames::shrink_png`]).
fn shrink_png_with_mask(
    bytes: &[u8],
    mask: Option<&[u8]>,
    decl: &crate::rules::art_data::PictureDecl,
) -> Result<GamePictureFrames, PictureArtError> {
    let canvas = decl.size.map(|size| size.map(|side| side.unsigned_abs()));
    GamePictureFrames::shrink_png(bytes, mask, canvas)
}

/// The picture `file` (`bytes`), with its mask beside it, shrunk to the
/// game pixels `decl` gives.
fn shrink_picture(
    file: &str,
    bytes: &[u8],
    decl: &crate::rules::art_data::PictureDecl,
    asset_manager: &crate::assets::asset_manager::AssetManager,
) -> Result<GamePictureFrames, PictureArtError> {
    shrink_png_with_mask(bytes, asset_manager.get_ref(&mask_name(file)), decl)
}

/// Log `source`'s picture `file` as decoded, or why it cannot be drawn, as
/// an art problem told in the match.
fn report_decoded(
    source: &PictureSource,
    file: &str,
    decoded: Result<GamePictureFrames, PictureArtError>,
) -> Option<GamePictureFrames> {
    let (section, decl) = (source.section.as_str(), &source.decl);
    match decoded {
        Ok(frames) => {
            log::info!(
                "[{section}] {file}: {} frames of {}x{}",
                frames.frame_count(),
                frames.canvas[0],
                frames.canvas[1]
            );
            Some(frames)
        }
        Err(error) => {
            log::warn!("[{section}] {file} cannot be drawn: {error}");
            crate::rules::art_problems::report(
                section,
                &format!("Picture={}", decl.file),
                &format!("{file} cannot be drawn ({error})"),
                "it draws nothing",
            );
            None
        }
    }
}

/// A destroyed building's rubble frames, for each of `arts` in order, when
/// its art declares `Picture=`: the body picture rubble binding resolved
/// (`RubbleArt::shp`), at game pixels. None for SHP rubble and for a picture
/// that cannot be drawn. Each declared picture is decoded and shrunk side by
/// side on the worker pool (each is a whole building body), then logged and
/// reported in `arts`' order. All None, and no pool work, when none declares
/// a picture.
pub(crate) fn rubble_pictures(
    arts: &[crate::rules::building_rubble::RubbleArt],
    asset_manager: &crate::assets::asset_manager::AssetManager,
) -> Vec<Option<GamePictureFrames>> {
    if arts.iter().all(|art| art.picture.is_none()) {
        return arts.iter().map(|_| None).collect();
    }
    let decoded = crate::sim::parallel::map(arts, |art| {
        let decl = art.picture.as_ref()?;
        let bytes = asset_manager.get_ref(&art.shp)?;
        Some(shrink_picture(&art.shp, bytes, decl, asset_manager))
    });
    arts.iter()
        .zip(decoded)
        .map(|(art, decoded)| {
            let source = PictureSource {
                section: art.shp.clone(),
                decl: art.picture.clone()?,
            };
            report_decoded(&source, &art.shp, decoded?)
        })
        .collect()
}

/// One RGBA source page of picture sprites: its size, pixels, index plane
/// and the sprites on it with their top-left corners.
pub(crate) struct PicturePage {
    pub size: [u32; 2],
    pub rgba: Vec<u8>,
    pub indices: Vec<u8>,
    pub sprites: Vec<(PlacedPicture, [u32; 2])>,
}

/// A strip page `scale` times larger: each frame's [`HdSprite`] at `scale`
/// times its place on the 1x page, so a UV on one page addresses the same
/// picture point on the other (the interface strips,
/// `CenteredShapeStrip::from_picture_on_device`).
pub(crate) struct HdPage {
    pub scale: u32,
    pub size: [u32; 2],
    /// Premultiplied RGBA, level 0.
    pub rgba: Vec<u8>,
    pub indices: Vec<u8>,
}

impl HdPage {
    /// The page's mip levels ([`mip_chain`]), consuming its level 0, and
    /// its index plane.
    pub(crate) fn into_mips(self) -> (Vec<u8>, u32, Vec<u8>) {
        let levels = mip_levels(self.scale, self.size);
        (
            mip_chain(self.rgba, self.size, levels),
            levels,
            self.indices,
        )
    }
}

/// The mip levels the zoomed draw samples of `scale` art `size` texels
/// large: down to one texel per game pixel (`log2(scale)` halvings), and no
/// further than one texel.
pub(crate) fn mip_levels(scale: u32, size: [u32; 2]) -> u32 {
    (scale.max(1).ilog2() + 1).min(size[0].max(size[1]).max(1).ilog2() + 1)
}

/// `levels` mip levels of the premultiplied `rgba`, `size` texels: each the
/// 2x2 mean of the level above (premultiplied, so holes do not darken
/// edges), their RGBA one after another, level 0 (`rgba` itself) first.
pub(crate) fn mip_chain(rgba: Vec<u8>, size: [u32; 2], levels: u32) -> Vec<u8> {
    let mut chain = rgba;
    let (mut start, mut size) = (0, size);
    for _ in 1..levels {
        let next = size.map(|side| (side / 2).max(1));
        let mut out = Vec::with_capacity((next[0] * next[1] * 4) as usize);
        let level = &chain[start..];
        for y in 0..next[1] {
            for x in 0..next[0] {
                let mut sum = [0u32; 4];
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let sx = (x * 2 + dx).min(size[0] - 1);
                    let sy = (y * 2 + dy).min(size[1] - 1);
                    let at = ((sy * size[0] + sx) * 4) as usize;
                    for channel in 0..4 {
                        sum[channel] += u32::from(level[at + channel]);
                    }
                }
                out.extend(sum.map(|total| ((total + 2) / 4) as u8));
            }
        }
        start = chain.len();
        chain.extend_from_slice(&out);
        size = next;
    }
    chain
}

/// The side picture pages are packed to: 4096, a 64 MiB RGBA page, so a
/// map's worth of pictures spreads over many pages instead of one or two
/// pages of a gigabyte each that the GPU must take in one upload; a larger
/// sprite takes a page of its own size. Never above the device's
/// `max_side`.
pub(crate) const PICTURE_PAGE_SIDE: u32 = 4096;

/// [`PICTURE_PAGE_SIDE`], or `pictures`' largest side above it, within
/// `max_side`.
pub(crate) fn picture_page_side(pictures: &[PlacedPicture], max_side: u32) -> u32 {
    let largest = pictures
        .iter()
        .map(|picture| picture.sprite.size[0].max(picture.sprite.size[1]))
        .max()
        .unwrap_or(0);
    PICTURE_PAGE_SIDE.max(largest).min(max_side)
}

/// Pack `pictures` onto pages no wider or taller than `max_side`, tallest
/// first in shelves. A sprite larger than a page is left out, logged. The
/// HD twins of art above 1x are not here: the zoomed draw makes them when it
/// needs them (`render::picture_hd`).
pub(crate) fn pack_picture_pages(
    mut pictures: Vec<PlacedPicture>,
    max_side: u32,
) -> Vec<PicturePage> {
    pictures.sort_by_key(|picture| std::cmp::Reverse(picture.sprite.size[1]));
    let mut placed: Vec<Vec<(PlacedPicture, [u32; 2])>> = Vec::new();
    let mut extents: Vec<[u32; 2]> = Vec::new();
    let (mut x, mut y, mut shelf) = (0u32, 0u32, 0u32);
    for picture in pictures {
        let [width, height] = picture.sprite.size;
        if width > max_side || height > max_side {
            log::warn!(
                "[{}] frame {} is {width}x{height}, larger than a {max_side} page",
                picture.key.type_id,
                picture.key.frame
            );
            crate::rules::art_problems::report(
                &picture.key.type_id,
                "Picture=",
                &format!("a frame is larger than a {max_side}x{max_side} page"),
                "such frames draw nothing",
            );
            continue;
        }
        if placed.is_empty() || x + width > max_side {
            (x, y, shelf) = (0, y + shelf, 0);
        }
        if placed.is_empty() || y + height > max_side {
            placed.push(Vec::new());
            extents.push([0, 0]);
            (x, y, shelf) = (0, 0, 0);
        }
        let extent = extents.last_mut().unwrap();
        *extent = [extent[0].max(x + width), extent[1].max(y + height)];
        placed.last_mut().unwrap().push((picture, [x, y]));
        x += width;
        shelf = shelf.max(height);
    }
    placed
        .into_iter()
        .zip(extents)
        .map(|(mut sprites, size)| {
            let mut rgba = vec![0; (size[0] * size[1] * 4) as usize];
            let mut indices = vec![0; (size[0] * size[1]) as usize];
            for (picture, at) in &mut sprites {
                let sprite = &mut picture.sprite;
                blit(
                    &mut rgba,
                    &mut indices,
                    size[0],
                    *at,
                    sprite.size,
                    &sprite.rgba,
                    &sprite.indices,
                );
                // On the page now; the entry keeps only its rectangle.
                sprite.rgba = Vec::new();
                sprite.indices = Vec::new();
            }
            PicturePage {
                size,
                rgba,
                indices,
                sprites,
            }
        })
        .collect()
}

/// Copy a `size` sprite's RGBA and index plane to `at` on a page
/// `page_width` wide.
fn blit(
    page_rgba: &mut [u8],
    page_indices: &mut [u8],
    page_width: u32,
    [x, y]: [u32; 2],
    [width, height]: [u32; 2],
    rgba: &[u8],
    indices: &[u8],
) {
    let width = width as usize;
    for row in 0..height {
        let to = ((y + row) * page_width + x) as usize;
        let from = (row as usize) * width;
        page_rgba[to * 4..(to + width) * 4].copy_from_slice(&rgba[from * 4..(from + width) * 4]);
        page_indices[to..to + width].copy_from_slice(&indices[from..from + width]);
    }
}

#[cfg(all(test, tests_render_a_r))]
pub(crate) mod tests {
    use super::*;

    fn picture(width: u32, height: u32, frames: Vec<Vec<u8>>) -> PictureFrames {
        PictureFrames {
            width,
            height,
            frames,
        }
    }

    /// `frames` of a `width` x `height` picture, with `mask` beside it, as
    /// the atlas decodes them from their files on a `canvas`.
    pub(crate) fn decoded(
        width: u32,
        height: u32,
        frames: &[Vec<u8>],
        mask: Option<&[Vec<u8>]>,
        canvas: [u32; 2],
    ) -> GamePictureFrames {
        let file = |frames: &[Vec<u8>]| -> SharedBytes {
            let bytes = crate::assets::picture_file::encode_frames(width, height, frames).unwrap();
            Arc::<[u8]>::from(bytes).into()
        };
        let source = PictureSource {
            section: "BOOM".into(),
            decl: crate::rules::art_data::PictureDecl {
                file: "boom.png".into(),
                size: Some(canvas.map(|side| side as i32)),
            },
        };
        GamePictureFrames::decode_files(&source, "boom.png", file(frames), mask.map(file)).unwrap()
    }

    /// A 2x picture shrinks onto its canvas: a game pixel is drawn when half
    /// its picture pixels are, takes their mean colour and its mask's class,
    /// and the sprite is cropped to the drawn pixels.
    #[test]
    fn a_2x_picture_shrinks_by_coverage_and_crops_like_an_shp_frame() {
        const RED: [u8; 4] = [200, 0, 0, 255];
        const BLUE: [u8; 4] = [0, 0, 100, 255];
        const NONE: [u8; 4] = [0; 4];
        // 4x2 picture pixels make a 2x1 canvas. Left game pixel: two red
        // and two blue (drawn, mean colour). Right: one red only (not drawn).
        let rows = [[RED, BLUE, RED, NONE], [BLUE, RED, NONE, NONE]];
        let frame: Vec<u8> = rows.concat().concat();
        let mask_rows = [
            [[0, 255, 0, 255]; 4],
            [[0, 255, 0, 255], [0, 0, 0, 255], [0; 4], [0; 4]],
        ];
        let mask: Vec<u8> = mask_rows.concat().concat();
        let shrunk = GamePictureFrames::shrink(
            &picture(4, 2, vec![frame]),
            Some(&picture(4, 2, vec![mask])),
            Some([2, 1]),
        )
        .unwrap();
        let sprite = shrunk.sprite(0, None).unwrap();
        assert_eq!((sprite.origin, sprite.size), ([0, 0], [1, 1]));
        assert_eq!(sprite.rgba, [100, 0, 50, 255]);
        assert_eq!(sprite.indices, [UNLIT_INDEX], "three of four are unlit");
        assert!(shrunk.sprite(1, None).is_none());
    }

    /// A house-colour pixel takes the owner's ramp step for its brightness,
    /// and a picture whose size is not a whole multiple of its canvas is
    /// refused.
    #[test]
    fn house_pixels_take_the_ramp_by_brightness_and_bad_scales_are_refused() {
        let ramp: [Color; RAMP_SIZE] = std::array::from_fn(|step| Color::rgb(step as u8, 0, 0));
        let frame = vec![255, 0, 0, 255, 0, 100, 0, 255];
        let mask = vec![255, 0, 0, 255, 255, 0, 0, 255];
        let shrunk = GamePictureFrames::shrink(
            &picture(2, 1, vec![frame.clone()]),
            Some(&picture(2, 1, vec![mask])),
            None,
        )
        .unwrap();
        assert!(shrunk.has_house_color());
        let sprite = shrunk.sprite(0, Some(&ramp)).unwrap();
        // 255 is step 0; 100 is (255 - 100) / 17 = step 9.
        assert_eq!(sprite.rgba, [0, 0, 0, 255, 9, 0, 0, 255]);
        assert_eq!(sprite.indices, [LIT_INDEX, LIT_INDEX]);
        assert!(
            GamePictureFrames::shrink(&picture(2, 1, vec![frame]), None, Some([3, 1])).is_err()
        );
    }

    fn placed(frame: u16, sprite: PictureSprite) -> PlacedPicture {
        PlacedPicture {
            key: crate::render::sprite_atlas::ShpSpriteKey {
                palette_context: crate::render::sprite_atlas::ShpPaletteContext::GlobalAnim,
                type_id: "BOOM".into(),
                facing: 0,
                frame,
                house_color: crate::rules::house_colors::HouseColorIndex(0),
            },
            sprite,
            offset: [0.0; 2],
            canvas_rect: [0.0; 4],
            extended: false,
            hd: None,
        }
    }

    /// Sprites pack in shelves, tallest first, onto as few pages as fit, and
    /// each page holds their pixels and index planes where they were placed.
    #[test]
    fn pictures_pack_in_shelves_and_keep_their_pixels() {
        let sprite = |frame: u16, size: [u32; 2], value: u8| {
            placed(
                frame,
                PictureSprite {
                    origin: [0, 0],
                    size,
                    rgba: vec![value; (size[0] * size[1] * 4) as usize],
                    indices: vec![LIT_INDEX; (size[0] * size[1]) as usize],
                },
            )
        };
        let pages = pack_picture_pages(
            vec![
                sprite(0, [3, 1], 10),
                sprite(1, [2, 2], 20),
                sprite(2, [4, 4], 30),
            ],
            4,
        );
        let placements: Vec<Vec<(u16, [u32; 2])>> = pages
            .iter()
            .map(|page| {
                page.sprites
                    .iter()
                    .map(|(p, at)| (p.key.frame, *at))
                    .collect()
            })
            .collect();
        assert_eq!(
            placements,
            [vec![(2, [0, 0])], vec![(1, [0, 0]), (0, [0, 2])]]
        );
        assert_eq!(pages[1].size, [3, 3]);
        // Frame 0 sits on row 2 of the second page; nothing is beside frame 1.
        assert_eq!(pages[1].rgba[(2 * 3) * 4], 10);
        assert_eq!(pages[1].indices[2], 0);
    }

    /// More picture sprites than one page holds spread over as many pages
    /// as they need, every one placed once; pages are 4096 square unless a
    /// sprite is larger, and never past the device's limit.
    #[test]
    fn picture_sprites_beyond_one_page_all_get_pages() {
        let sprite = |frame: u16, side: u32| {
            placed(
                frame,
                PictureSprite {
                    origin: [0, 0],
                    size: [side, side],
                    rgba: vec![9; (side * side * 4) as usize],
                    indices: vec![LIT_INDEX; (side * side) as usize],
                },
            )
        };
        // Four 3x3 sprites fit an 8x8 page; twenty need five pages.
        let pages = pack_picture_pages((0..20).map(|frame| sprite(frame, 3)).collect(), 8);
        let mut frames: Vec<u16> = pages
            .iter()
            .flat_map(|page| page.sprites.iter().map(|(picture, _)| picture.key.frame))
            .collect();
        frames.sort_unstable();
        assert_eq!(frames, (0..20).collect::<Vec<_>>());
        assert_eq!(pages.len(), 5);
        assert!(
            pages
                .iter()
                .all(|page| page.size[0] <= 8 && page.size[1] <= 8)
        );

        assert_eq!(picture_page_side(&[sprite(0, 3)], 16384), PICTURE_PAGE_SIDE);
        assert_eq!(picture_page_side(&[sprite(0, 5000)], 16384), 5000);
        assert_eq!(picture_page_side(&[sprite(0, 3)], 2048), 2048);
    }

    /// A 4x picture is decoded to its game pixels only: its sprite keeps a
    /// request for its HD twin (the picture's files, frame and rectangle),
    /// and it packs with 1x art onto one page of game pixels, with no twin.
    #[test]
    fn a_4x_picture_keeps_game_pixels_and_a_twin_request_only() {
        const RED: [u8; 4] = [200, 0, 0, 255];
        // An 8x4 picture on a 2x1 canvas: the right game pixel is drawn.
        let frame: Vec<u8> = (0..32)
            .flat_map(|at| if at % 8 >= 4 { RED } else { [0; 4] })
            .collect();
        let frames = decoded(8, 4, &[frame], None, [2, 1]);
        assert_eq!(frames.scale, 4);
        assert_eq!(frames.frames[0].rgba.len(), 2 * 4, "game pixels only");
        let sprite = frames.sprite(0, None).unwrap();
        assert_eq!((sprite.origin, sprite.size), ([1, 0], [1, 1]));
        let twin = frames.hd_twin(0, &sprite, None).unwrap();
        assert_eq!((twin.frame, twin.origin, twin.size), (0, [1, 0], [1, 1]));
        assert_eq!(twin.files.scale, 4);

        let mut hd = placed(0, sprite.clone());
        hd.hd = Some(twin);
        let pages = pack_picture_pages(vec![hd, placed(1, sprite)], 8);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].size, [2, 1], "two game pixels, no gap");
        assert_eq!(pages[0].rgba.len(), 2 * 4);
    }

    /// A picture frame takes the format its SHP would store: RLE (the
    /// extended walker) when the RLE rows are smaller than the raw
    /// rectangle, as for an anim's sparse arcs, else raw, as for a dense
    /// frame; a hole run longer than 255 takes two runs.
    #[test]
    fn a_picture_frame_takes_the_format_its_shp_would_store() {
        let sprite = |size: [u32; 2], indices: Vec<u8>| PictureSprite {
            origin: [0, 0],
            size,
            rgba: vec![0; indices.len() * 4],
            indices,
        };
        // Dense: 2 + 4 bytes against 4 raw.
        assert!(!sprite([4, 1], vec![LIT_INDEX; 4]).stored_as_rle());
        // One drawn pixel and nine holes: 2 + 1 + 2 against 10.
        let mut sparse = vec![0; 10];
        sparse[0] = LIT_INDEX;
        assert!(sprite([10, 1], sparse).stored_as_rle());
        // Two drawn pixels around 300 holes: 2 + 1 + 2 + 2 + 1 = 8.
        let mut long = vec![0; 302];
        (long[0], long[301]) = (LIT_INDEX, LIT_INDEX);
        assert!(sprite([302, 1], long).stored_as_rle());
        // Three rows of 2 + 2 against 6 raw: the length words make it raw.
        assert!(!sprite([2, 3], vec![LIT_INDEX; 6]).stored_as_rle());
    }

    /// Mips average premultiplied texels down to one per game pixel, so a
    /// hole does not darken its drawn neighbour.
    #[test]
    fn hd_mips_average_premultiplied_texels_down_to_game_pixels() {
        let rgba = [[200, 0, 0, 255], [0; 4], [0; 4], [200, 0, 0, 255]].concat();
        let levels = mip_levels(2, [2, 2]);
        assert_eq!(levels, 2);
        let chain = mip_chain(rgba.clone(), [2, 2], levels);
        assert_eq!(chain[..16], rgba[..]);
        // Premultiplied mean: colour 100 at alpha 128 is 200 once divided.
        assert_eq!(chain[16..], [100, 0, 0, 128]);
    }
}
