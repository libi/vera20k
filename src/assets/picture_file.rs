//! VERA-only, experimental picture art (`Picture=`,
//! `docs/picture-model-art.md`): the header facts of a PNG or APNG that the
//! simulation takes where it takes an SHP header's, read without decoding a
//! pixel.

/// A picture's canvas, in its own pixels, and its frame count: the APNG's
/// animation frames (`acTL`), or 1 for a plain PNG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PictureHeader {
    pub width: u32,
    pub height: u32,
    pub frames: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum PictureError {
    #[error("not a readable PNG: {0}")]
    Png(#[from] png::DecodingError),
    #[error("an APNG with no frames")]
    NoFrames,
    #[error("{0} bytes of frames is more than the {PICTURE_BYTES_MAX} a picture may hold")]
    TooLarge(u64),
    #[error("a {0}-byte frame is more than the {PICTURE_FRAME_BYTES_MAX} a frame may hold")]
    FrameTooLarge(u64),
    #[error("a frame region outside the canvas")]
    FrameOutside,
}

/// The most decoded RGBA bytes [`PictureFrames::decode`] may hold, all
/// frames together: the small pictures kept whole (the interface strips, a
/// cameo, the placement preview). The atlas reads a picture one frame at a
/// time instead ([`PictureFrameReader`]).
pub const PICTURE_BYTES_MAX: u64 = 512 << 20;

/// The most decoded RGBA bytes one frame's canvas may hold (8192 square):
/// what a reader holds at once, about three canvases with its row buffer
/// and an APNG Previous disposal's copy.
pub const PICTURE_FRAME_BYTES_MAX: u64 = 256 << 20;

/// A picture's frames composited onto its canvas, as an APNG viewer shows
/// them: straight RGBA, row by row, `width * height * 4` bytes each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PictureFrames {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<Vec<u8>>,
}

impl PictureFrames {
    /// Decode and composite every frame of the PNG or APNG `bytes`
    /// ([`PictureFrameReader`]), all frames kept.
    pub fn decode(bytes: &[u8]) -> Result<Self, PictureError> {
        let mut reader = PictureFrameReader::new(bytes)?;
        let (width, height) = reader.size();
        let total = u64::from(width) * u64::from(height) * 4 * u64::from(reader.frame_count());
        if total > PICTURE_BYTES_MAX {
            return Err(PictureError::TooLarge(total));
        }
        let mut frames = Vec::with_capacity(reader.frame_count() as usize);
        while let Some(frame) = reader.next_frame()? {
            frames.push(frame.to_vec());
        }
        Ok(Self {
            width,
            height,
            frames,
        })
    }
}

/// A PNG or APNG's frames composited onto its canvas one at a time, as an
/// APNG viewer shows them, so a reader that needs each frame only once holds
/// one canvas instead of every frame. An APNG's default image that is not
/// part of its animation is skipped; each animation frame is drawn onto the
/// canvas by its blend operation and then disposed of by its dispose
/// operation (APNG 1.0, `fcTL`) before the next one.
pub struct PictureFrameReader<'a> {
    reader: png::Reader<std::io::Cursor<&'a [u8]>>,
    width: u32,
    height: u32,
    frame_count: u32,
    next: u32,
    buffer: Vec<u8>,
    canvas: Vec<u8>,
    /// What the frame last shown asks done to the canvas before the next.
    dispose: Option<(png::DisposeOp, [u32; 4], Option<Vec<u8>>)>,
}

impl<'a> PictureFrameReader<'a> {
    /// The reader of `bytes`, its header read. Refuses a picture whose canvas
    /// exceeds [`PICTURE_FRAME_BYTES_MAX`] decoded: the reader holds one
    /// frame, so the frame count is not bounded here.
    pub fn new(bytes: &'a [u8]) -> Result<Self, PictureError> {
        let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let info = reader.info();
        let (width, height) = (info.width, info.height);
        let animation = info.animation_control;
        let frame_count = animation.map_or(1, |control| control.num_frames);
        if frame_count == 0 {
            return Err(PictureError::NoFrames);
        }
        let canvas_bytes = u64::from(width) * u64::from(height) * 4;
        if canvas_bytes > PICTURE_FRAME_BYTES_MAX {
            return Err(PictureError::FrameTooLarge(canvas_bytes));
        }
        let default_in_animation = animation.is_none() || info.frame_control.is_some();
        let mut buffer = vec![
            0;
            reader
                .output_buffer_size()
                .ok_or(PictureError::FrameTooLarge(canvas_bytes))?
        ];
        if !default_in_animation {
            reader.next_frame(&mut buffer)?;
        }
        Ok(Self {
            reader,
            width,
            height,
            frame_count,
            next: 0,
            buffer,
            canvas: vec![0; canvas_bytes as usize],
            dispose: None,
        })
    }

    /// The canvas, in picture pixels.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    /// The next frame composited onto the canvas, straight RGBA, `width *
    /// height * 4` bytes; None after the last.
    pub fn next_frame(&mut self) -> Result<Option<&[u8]>, PictureError> {
        if self.next >= self.frame_count {
            return Ok(None);
        }
        let (width, height) = (self.width, self.height);
        match self.dispose.take() {
            None | Some((png::DisposeOp::None, ..)) => {}
            Some((png::DisposeOp::Background, [x, y, w, h], _)) => {
                for row in y..y + h {
                    let start = ((row * width + x) * 4) as usize;
                    self.canvas[start..start + (w * 4) as usize].fill(0);
                }
            }
            Some((png::DisposeOp::Previous, _, saved)) => {
                if let Some(saved) = saved {
                    self.canvas = saved;
                }
            }
        }
        let index = self.next;
        self.next += 1;
        let output = self.reader.next_frame(&mut self.buffer)?;
        let control = self.reader.info().frame_control;
        let region = control.map_or([0, 0, width, height], |control| {
            [
                control.x_offset,
                control.y_offset,
                control.width,
                control.height,
            ]
        });
        let [x, y, w, h] = region;
        if x.checked_add(w).is_none_or(|right| right > width)
            || y.checked_add(h).is_none_or(|bottom| bottom > height)
        {
            return Err(PictureError::FrameOutside);
        }
        let samples = &self.buffer[..output.buffer_size()];
        // RGBA output is used as it is; other layouts are expanded.
        let expanded;
        let rgba: &[u8] = if output.color_type == png::ColorType::Rgba {
            samples
        } else {
            expanded = to_rgba(samples, output.color_type, w, h);
            &expanded
        };
        let dispose = control.map_or(png::DisposeOp::None, |control| control.dispose_op);
        // A first frame's Previous disposal acts as Background (APNG 1.0).
        let dispose = match (index, dispose) {
            (0, png::DisposeOp::Previous) => png::DisposeOp::Background,
            (_, dispose) => dispose,
        };
        let saved = (dispose == png::DisposeOp::Previous).then(|| self.canvas.clone());
        let over = control.is_some_and(|control| control.blend_op == png::BlendOp::Over);
        let row_bytes = (w * 4) as usize;
        for row in 0..h {
            let from = (row * w * 4) as usize;
            let to = (((y + row) * width + x) * 4) as usize;
            let source = &rgba[from..from + row_bytes];
            let target = &mut self.canvas[to..to + row_bytes];
            if !over {
                target.copy_from_slice(source);
                continue;
            }
            for (source, target) in source.chunks_exact(4).zip(target.chunks_exact_mut(4)) {
                match source[3] {
                    255 => target.copy_from_slice(source),
                    0 => {}
                    _ => {
                        let blended =
                            blend_over(source.try_into().unwrap(), (&*target).try_into().unwrap());
                        target.copy_from_slice(&blended);
                    }
                }
            }
        }
        self.dispose = Some((dispose, region, saved));
        Ok(Some(&self.canvas))
    }
}

/// `w` x `h` decoded 8-bit samples of `color` as straight RGBA.
fn to_rgba(samples: &[u8], color: png::ColorType, w: u32, h: u32) -> Vec<u8> {
    let pixels = (w * h) as usize;
    let mut rgba = Vec::with_capacity(pixels * 4);
    let pixel = |index: usize| -> [u8; 4] {
        match color {
            png::ColorType::Grayscale => {
                let v = samples[index];
                [v, v, v, 255]
            }
            png::ColorType::GrayscaleAlpha => {
                let v = samples[index * 2];
                [v, v, v, samples[index * 2 + 1]]
            }
            png::ColorType::Rgb => {
                let s = &samples[index * 3..index * 3 + 3];
                [s[0], s[1], s[2], 255]
            }
            // Indexed is expanded to RGB or RGBA by `normalize_to_color8`.
            png::ColorType::Rgba | png::ColorType::Indexed => {
                samples[index * 4..index * 4 + 4].try_into().unwrap()
            }
        }
    };
    for index in 0..pixels {
        rgba.extend_from_slice(&pixel(index));
    }
    rgba
}

/// Straight-alpha `source` over `target` (APNG `APNG_BLEND_OP_OVER`).
fn blend_over(source: [u8; 4], target: [u8; 4]) -> [u8; 4] {
    let source_alpha = u32::from(source[3]);
    if source_alpha == 255 {
        return source;
    }
    if source_alpha == 0 {
        return target;
    }
    let target_alpha = u32::from(target[3]) * (255 - source_alpha) / 255;
    let alpha = source_alpha + target_alpha;
    let channel = |s: u8, t: u8| {
        ((u32::from(s) * source_alpha + u32::from(t) * target_alpha + alpha / 2) / alpha) as u8
    };
    [
        channel(source[0], target[0]),
        channel(source[1], target[1]),
        channel(source[2], target[2]),
        alpha as u8,
    ]
}

impl PictureFrames {
    /// The top-left `size` game pixels of frame `frame`, `scale` picture
    /// pixels per game pixel on each axis, as straight RGBA: a game pixel is
    /// drawn (alpha 255) when at least half its picture pixels are (alpha at
    /// least half), its colour their mean; else it is a hole (all 0). The
    /// coverage rule `render::picture_atlas` shrinks a picture by, without
    /// its mask. None for a frame past the end or a region outside the
    /// canvas.
    pub fn shrunk_region(&self, frame: usize, scale: u32, size: [u32; 2]) -> Option<Vec<u8>> {
        let rgba = self.frames.get(frame)?;
        if scale == 0 || size[0] * scale > self.width || size[1] * scale > self.height {
            return None;
        }
        let mut out = Vec::with_capacity((size[0] * size[1] * 4) as usize);
        for y in 0..size[1] {
            for x in 0..size[0] {
                let (mut covered, mut sum) = (0u32, [0u32; 3]);
                for dy in 0..scale {
                    for dx in 0..scale {
                        let at = (((y * scale + dy) * self.width + x * scale + dx) * 4) as usize;
                        if u32::from(rgba[at + 3]) * 2 >= 255 {
                            covered += 1;
                            for channel in 0..3 {
                                sum[channel] += u32::from(rgba[at + channel]);
                            }
                        }
                    }
                }
                if covered * 2 >= scale * scale {
                    let mean = |channel: usize| ((sum[channel] + covered / 2) / covered) as u8;
                    out.extend_from_slice(&[mean(0), mean(1), mean(2), 255]);
                } else {
                    out.extend_from_slice(&[0; 4]);
                }
            }
        }
        Some(out)
    }

    /// The mean colour of frame `frame`'s drawn pixels (alpha at least half),
    /// where an SHP frame header holds the radar colour the art author chose
    /// (`+0x0C..0x0E`). None for a frame past the end or with nothing drawn.
    pub fn mean_color(&self, frame: usize) -> Option<[u8; 3]> {
        let (mut count, mut sum) = (0u64, [0u64; 3]);
        for pixel in self.frames.get(frame)?.chunks_exact(4) {
            if u32::from(pixel[3]) * 2 >= 255 {
                count += 1;
                for channel in 0..3 {
                    sum[channel] += u64::from(pixel[channel]);
                }
            }
        }
        (count > 0).then(|| sum.map(|total| ((total + count / 2) / count) as u8))
    }
}

impl PictureHeader {
    /// The header of the PNG or APNG `bytes`.
    pub fn read(bytes: &[u8]) -> Result<Self, PictureError> {
        let reader = png::Decoder::new(std::io::Cursor::new(bytes)).read_info()?;
        let info = reader.info();
        let frames = info
            .animation_control
            .map_or(1, |control| control.num_frames);
        if frames == 0 {
            return Err(PictureError::NoFrames);
        }
        Ok(Self {
            width: info.width,
            height: info.height,
            frames,
        })
    }
}

/// Encode `frames`, each `width * height * 4` straight RGBA bytes, as a PNG
/// (one frame) or an APNG of whole-canvas frames that replace the canvas.
pub fn encode_frames(
    width: u32,
    height: u32,
    frames: &[Vec<u8>],
) -> Result<Vec<u8>, png::EncodingError> {
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    if frames.len() > 1 {
        let count = u32::try_from(frames.len()).unwrap_or(u32::MAX);
        encoder.set_animated(count, 0)?;
    }
    let mut writer = encoder.write_header()?;
    for frame in frames {
        writer.write_image_data(frame)?;
    }
    writer.finish()?;
    Ok(bytes)
}

/// An RGBA PNG of `width` x `height`, with `frames` APNG frames when some,
/// for tests of picture readers.
#[cfg(test)]
pub(crate) fn test_picture(width: u32, height: u32, frames: Option<u32>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    if let Some(frames) = frames {
        encoder.set_animated(frames, 0).unwrap();
    }
    let mut writer = encoder.write_header().unwrap();
    let pixels = vec![0u8; (width * height * 4) as usize];
    for _ in 0..frames.unwrap_or(1) {
        writer.write_image_data(&pixels).unwrap();
    }
    writer.finish().unwrap();
    bytes
}

#[cfg(all(test, tests_assets))]
mod tests {
    use super::*;

    #[test]
    fn an_apng_counts_its_animation_frames_and_a_png_one() {
        let apng = PictureHeader::read(&test_picture(8, 6, Some(5))).unwrap();
        assert_eq!(
            apng,
            PictureHeader {
                width: 8,
                height: 6,
                frames: 5
            }
        );
        assert_eq!(
            PictureHeader::read(&test_picture(3, 4, None))
                .unwrap()
                .frames,
            1
        );
        assert!(PictureHeader::read(b"not a png").is_err());
    }

    /// `bytes` (a PNG or APNG) with its IHDR size, and its first frame
    /// control's, set to `width` x `height`; no pixel data changes.
    fn resized(mut bytes: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
        // Each chunk: length, type, data, CRC-32 of type and data (PNG 1.2,
        // 5.3). The size sits at data offset 0 in IHDR, 4 in fcTL.
        for (kind, at) in [(&b"IHDR"[..], 0), (&b"fcTL"[..], 4)] {
            let Some(start) = bytes.windows(4).position(|window| window == kind) else {
                continue;
            };
            let length = u32::from_be_bytes(bytes[start - 4..start].try_into().unwrap()) as usize;
            let data = start + 4;
            bytes[data + at..data + at + 4].copy_from_slice(&width.to_be_bytes());
            bytes[data + at + 4..data + at + 8].copy_from_slice(&height.to_be_bytes());
            let mut crc = !0u32;
            for &byte in &bytes[start..data + length] {
                crc ^= u32::from(byte);
                for _ in 0..8 {
                    crc = if crc & 1 != 0 {
                        (crc >> 1) ^ 0xEDB8_8320
                    } else {
                        crc >> 1
                    };
                }
            }
            bytes[data + length..data + length + 4].copy_from_slice(&(!crc).to_be_bytes());
        }
        bytes
    }

    /// The reader holds one frame, so only the canvas is bounded: an APNG
    /// whose frames together pass the whole-picture cap reads frame by frame
    /// and is refused only where it would be kept whole; a canvas past the
    /// frame cap is refused outright.
    #[test]
    fn pictures_are_bounded_per_frame_and_by_total_only_when_kept_whole() {
        // 4096 square is 64 MiB a frame; 40 frames are 2.5 GiB.
        let tall = resized(test_picture(1, 1, Some(40)), 4096, 4096);
        let reader = PictureFrameReader::new(&tall).unwrap();
        assert_eq!((reader.size(), reader.frame_count()), ((4096, 4096), 40));
        assert!(matches!(
            PictureFrames::decode(&tall),
            Err(PictureError::TooLarge(_))
        ));
        let huge = resized(test_picture(1, 1, None), 16384, 16384);
        assert!(matches!(
            PictureFrameReader::new(&huge),
            Err(PictureError::FrameTooLarge(_))
        ));
    }

    /// Frames that cover part of the canvas composite as an APNG viewer
    /// shows them: Source replaces, Over blends, Background clears after.
    #[test]
    fn apng_frames_composite_by_blend_and_dispose() {
        const RED: [u8; 4] = [255, 0, 0, 255];
        const GREEN: [u8; 4] = [0, 255, 0, 255];
        const CLEAR: [u8; 4] = [0; 4];
        let mut bytes = Vec::new();
        let mut encoder = png::Encoder::new(&mut bytes, 4, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_animated(3, 0).unwrap();
        let mut writer = encoder.write_header().unwrap();
        // Frame 0: the whole row red, cleared to nothing after it shows.
        writer.set_dispose_op(png::DisposeOp::Background).unwrap();
        writer.write_image_data(&RED.repeat(4)).unwrap();
        // Frame 1: green over the right half only.
        writer.set_dispose_op(png::DisposeOp::None).unwrap();
        writer.set_frame_dimension(2, 1).unwrap();
        writer.set_frame_position(2, 0).unwrap();
        writer.write_image_data(&GREEN.repeat(2)).unwrap();
        // Frame 2: half-transparent red blended over pixel 3.
        writer.set_blend_op(png::BlendOp::Over).unwrap();
        writer.set_frame_dimension(1, 1).unwrap();
        writer.set_frame_position(3, 0).unwrap();
        writer.write_image_data(&[255, 0, 0, 128]).unwrap();
        writer.finish().unwrap();

        let picture = PictureFrames::decode(&bytes).unwrap();
        assert_eq!((picture.width, picture.height), (4, 1));
        assert_eq!(picture.frames[0], RED.repeat(4));
        assert_eq!(picture.frames[1], [CLEAR, CLEAR, GREEN, GREEN].concat());
        assert_eq!(
            &picture.frames[2][..12],
            &[CLEAR, CLEAR, GREEN].concat()[..]
        );
        assert_eq!(&picture.frames[2][12..], &[128, 127, 0, 255]);
    }
}
