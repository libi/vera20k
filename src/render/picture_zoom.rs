//! VERA-only, experimental (`docs/picture-model-art.md`, chain 6): picture
//! art drawn again at its own resolution over the magnified world.
//!
//! The world draws every picture at `PictureSize=` game pixels, in native
//! order with everything else, and the wheel zoom or the Options zoom then
//! magnifies it. Art above 1x has HD twins of its sprites, made when this
//! pass first needs them (`render::picture_hd`), each instance's HD UV
//! rectangle beside it; the interface strips and the model page keep twins
//! of their pages at the same normalised coordinates, where the HD UV is the
//! instance's own. After the magnification this pass draws each opaque
//! world instance with a twin again, from the twin, at the target's
//! resolution; one whose twin is not ready keeps its magnified pixels. Its shader
//! (`picture_zoom.wgsl`) maps each target pixel back to the world pixel it
//! shows, computes that pixel's native Z as the zsprite shader does and
//! compares it with the world's own depth there (no copied depth), so the
//! sharp pixels replace the magnified ones exactly where the 1x draw passed.
//!
//! A sharp pixel replaces a magnified one only where the completed world
//! still shows the instance's own 1x pixel: the shader recomputes what the 1x
//! draw stored there from the page itself and compares it with the world's
//! colour, so anything drawn over the object later in the world without
//! writing Z (units in front, smoke, explosions, light) keeps its magnified
//! pixels. Translucent, FX and packed (destination-reading) draws keep
//! their magnified 1x pixels. The WebGL2 fallback cannot load a depth texture
//! in a shader, so it builds no HD pages and the pass never runs there.
//!
//! The interface strips drawn from pictures (`render::interface_picture`:
//! status markers, pips, health bars, the placement grid, beacons) have HD
//! twins of their own ([`HdSource::Interface`]); their copies follow the
//! world's, in the order the world drew them.
//!
//! Nothing here exists, allocates or walks anything unless the sprite atlas
//! has art with HD twins or an interface strip has an HD page
//! (`SpriteAtlas::has_picture_hd_pages`, `SelectionOverlay::has_hd_pages`,
//! `BeaconArt::has_hd_pages`).

use crate::render::batch::{
    BatchRenderer, BatchTexture, SPRITE_INSTANCE_ATTRIBUTES, SpriteInstance,
};
use crate::render::picture_atlas::HdPage;

/// One HD page on the GPU: premultiplied encoded colour with mipmaps down to
/// one texel per game pixel, and its level-0 index plane.
pub(crate) struct PictureHdPage {
    color: wgpu::TextureView,
    indices: wgpu::TextureView,
}

impl PictureHdPage {
    /// A page drawn on the GPU (a model page's HD twin,
    /// `render::model_draw`): encoded colour that is opaque or empty, so
    /// already premultiplied, with one level, and its index plane.
    pub(crate) fn from_views(color: wgpu::TextureView, indices: wgpu::TextureView) -> Self {
        Self { color, indices }
    }

    /// The colour, encoded bytes (premultiplied, which an opaque or empty
    /// texel leaves as it is), as the terrain HD pass reads its pages.
    pub(crate) fn color_view(&self) -> &wgpu::TextureView {
        &self.color
    }

    pub(crate) fn new(device: &wgpu::Device, queue: &wgpu::Queue, page: HdPage) -> Self {
        use wgpu::util::DeviceExt;
        let size = wgpu::Extent3d {
            width: page.size[0],
            height: page.size[1],
            depth_or_array_layers: 1,
        };
        let (chain, levels, indices) = page.into_mips();
        let texture = |label, format, mip_level_count, data: &[u8]| {
            device
                .create_texture_with_data(
                    queue,
                    &wgpu::TextureDescriptor {
                        label: Some(label),
                        size,
                        mip_level_count,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING,
                        view_formats: &[],
                    },
                    wgpu::util::TextureDataOrder::LayerMajor,
                    data,
                )
                .create_view(&Default::default())
        };
        Self {
            // Encoded bytes, as the palette rows: the shader divides by alpha
            // and hands the result to the palette light unchanged.
            color: texture(
                "Picture HD page",
                wgpu::TextureFormat::Rgba8Unorm,
                levels,
                &chain,
            ),
            indices: texture(
                "Picture HD page indices",
                wgpu::TextureFormat::R8Uint,
                1,
                &indices,
            ),
        }
    }
}

/// How an HD run tests Z: the world run's `RenderZPolicy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HdDepth {
    /// Passthrough: no test.
    None = 0,
    /// Read-only leaves: native Less.
    Less = 1,
    /// Read-write leaves, which stored their own Z: Less or equal.
    LessEqual = 2,
}

/// The page a run of HD instances draws from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HdSource {
    /// A sprite atlas page with a twin at its own UV
    /// (`SpriteAtlas::picture_hd_page`: the model page).
    Atlas(usize),
    /// Picture HD page `hd` (`render::picture_hd`), for instances the world
    /// drew from atlas page `page`.
    Streamed { hd: usize, page: usize },
    /// An interface strip drawn from a picture.
    Interface(crate::render::selection_overlay::InterfaceStrip),
}

/// A run of HD instances drawn from one page with one Z test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HdRun {
    pub page: HdSource,
    pub depth: HdDepth,
    /// The world draw bound the surface camera, whose A plane is neutral
    /// (`BatchRenderer::surface_camera_bind_group`): stores that bypass A.
    pub neutral_a: bool,
    pub start: u32,
    pub count: u32,
}

/// Where this frame's HD draw goes: over the magnified frame
/// (`WorldZoomPass::composite`), or over the window when the Options zoom
/// scales the frame and the world goes there itself
/// (`WorldZoomPass::draw_window_copy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HdTarget {
    Frame,
    Window,
}

/// Where the world lands in a target, in target pixels.
pub(crate) struct HdPlacement<'a> {
    /// The target pixel of world-target pixel (0, 0).
    pub origin: [f32; 2],
    /// Target pixels per world pixel; the pass draws only above 1.
    pub scale: f32,
    /// For the window only: what keeps the frame's own pixels.
    pub mask: Option<HdMask<'a>>,
}

/// A window pixel whose frame pixel changed after the world went in (UI
/// drawn over the battlefield) keeps the window, as the window copy's
/// `WindowMask::Changed` keeps it.
pub(crate) struct HdMask<'a> {
    /// The window pixel of frame pixel (0, 0), and window pixels per frame
    /// pixel.
    pub frame_origin: [f32; 2],
    pub frame_scale: f32,
    /// The finished frame, and the frame as the world left it.
    pub now: &'a wgpu::TextureView,
    pub before: &'a wgpu::TextureView,
}

/// `Params` of picture_zoom.wgsl.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Params {
    origin: [f32; 2],
    scale: f32,
    policy: u32,
    target_size: [f32; 2],
    frame_scale: f32,
    masked: u32,
    frame_origin: [f32; 2],
    pad: [f32; 2],
}

const PARAMS_SIZE: u64 = std::mem::size_of::<Params>() as u64;

/// The pass: its pipeline, and this frame's HD instances and runs.
pub(crate) struct PictureZoomPass {
    pipeline: wgpu::RenderPipeline,
    page_layout: wgpu::BindGroupLayout,
    view_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    /// One `Params` per [`HdDepth`], `params_stride` apart (dynamic offset).
    params: wgpu::Buffer,
    params_stride: u64,
    /// Bound as both mask textures where there is no mask.
    no_mask: wgpu::TextureView,
    instances: Option<wgpu::Buffer>,
    /// Each instance's UV rectangle on its HD page (origin, size).
    hd_uvs: Option<wgpu::Buffer>,
    runs: Vec<HdRun>,
    target: HdTarget,
    /// The tile pictures' twins this frame draws, by HD page of the tile
    /// atlas's store (`render::hd_terrain`'s pass).
    tiles: Vec<(usize, Vec<TileHdDraw>)>,
    terrain: crate::render::hd_terrain::HdTerrainPass,
}

/// A cell drawn again from its tile picture's twin: the terrain instance's
/// world position and size, and its places in the tile atlas and on the HD
/// page, and the twin's scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TileHdDraw {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub atlas_origin: [u32; 2],
    pub hd_origin: [u32; 2],
    pub scale: u32,
}

/// The instance attributes picture_zoom.wgsl reads; the HD UV rectangle
/// takes location 4, the depth it does not read.
const HD_UV_LOCATION: u32 = 4;
const READ_LOCATIONS: [u32; 11] = [0, 1, 2, 3, 5, 6, 8, 11, 12, 13, 14];

impl PictureZoomPass {
    pub(crate) fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        batch: &BatchRenderer,
    ) -> Self {
        use wgpu::util::DeviceExt;
        let texture_entry = |binding, sample_type| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type,
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let page_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Picture HD page BGL"),
            entries: &[
                texture_entry(0, wgpu::TextureSampleType::Float { filterable: true }),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                texture_entry(2, wgpu::TextureSampleType::Uint),
                // The page as the world drew from it (`own_world_pixel`).
                texture_entry(3, wgpu::TextureSampleType::Float { filterable: false }),
                texture_entry(4, wgpu::TextureSampleType::Uint),
            ],
        });
        let frame = wgpu::TextureSampleType::Float { filterable: false };
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Picture HD view BGL"),
            entries: &[
                texture_entry(0, wgpu::TextureSampleType::Depth),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(PARAMS_SIZE),
                    },
                    count: None,
                },
                texture_entry(2, frame),
                texture_entry(3, frame),
                texture_entry(4, frame),
            ],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Picture HD shader"),
            source: wgpu::ShaderSource::Wgsl(
                crate::render::tactical_shader::world_source(include_str!("picture_zoom.wgsl"))
                    .into(),
            ),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Picture HD pipeline layout"),
            bind_group_layouts: &[
                Some(batch.camera_bind_group_layout()),
                Some(&page_layout),
                Some(&view_layout),
                Some(batch.zshape_bind_group_layout()),
            ],
            immediate_size: 0,
        });
        let attributes: Vec<wgpu::VertexAttribute> = SPRITE_INSTANCE_ATTRIBUTES
            .iter()
            .filter(|attribute| READ_LOCATIONS.contains(&attribute.shader_location))
            .copied()
            .collect();
        let hd_uv = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 0,
            shader_location: HD_UV_LOCATION,
        }];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Picture HD pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<SpriteInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &attributes,
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 4]>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &hd_uv,
                    }),
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                // Opaque stores only (`app::presentation::render::picture_zoom`).
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Picture HD sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let params_stride = PARAMS_SIZE.next_multiple_of(u64::from(
            device.limits().min_uniform_buffer_offset_alignment,
        ));
        let params = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Picture HD params"),
            size: params_stride * 3,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let no_mask = device
            .create_texture_with_data(
                queue,
                &wgpu::TextureDescriptor {
                    label: Some("Picture HD no mask"),
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
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &[0; 4],
            )
            .create_view(&Default::default());
        Self {
            pipeline,
            page_layout,
            view_layout,
            sampler,
            params,
            params_stride,
            no_mask,
            instances: None,
            hd_uvs: None,
            runs: Vec::new(),
            target: HdTarget::Frame,
            tiles: Vec::new(),
            terrain: {
                let mut terrain = crate::render::hd_terrain::HdTerrainPass::new(device);
                terrain.ensure_pipeline(device, format);
                terrain
            },
        }
    }

    /// This frame's HD `instances` with their HD UV rectangles `hd_uvs`,
    /// drawn in `runs`, and its `tiles`, for `target`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn set_frame(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instances: &[SpriteInstance],
        hd_uvs: &[[f32; 4]],
        runs: Vec<HdRun>,
        tiles: Vec<(usize, Vec<TileHdDraw>)>,
        target: HdTarget,
    ) {
        self.tiles = tiles;
        let upload = |buffer: &mut Option<wgpu::Buffer>, label, bytes: &[u8]| {
            let needed = (bytes.len() as u64).max(1);
            if buffer.as_ref().is_none_or(|buffer| buffer.size() < needed) {
                *buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size: needed.next_power_of_two().max(4096),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }));
            }
            if let Some(buffer) = buffer
                && !bytes.is_empty()
            {
                queue.write_buffer(buffer, 0, bytes);
            }
        };
        upload(
            &mut self.instances,
            "Picture HD instances",
            bytemuck::cast_slice(instances),
        );
        upload(
            &mut self.hd_uvs,
            "Picture HD UV rectangles",
            bytemuck::cast_slice(hd_uvs),
        );
        self.runs = runs;
        self.target = target;
    }

    /// No HD draw this frame.
    pub(crate) fn clear(&mut self) {
        self.runs.clear();
        self.tiles.clear();
    }

    /// Where this frame draws, if it draws.
    pub(crate) fn target(&self) -> Option<HdTarget> {
        (!self.runs.is_empty() || !self.tiles.is_empty()).then_some(self.target)
    }

    /// Draw this frame's tiles into `view` at `placement` inside `scissor`,
    /// before the sprites (`draw`), as the world drew its tiles first:
    /// `render::hd_terrain`'s pass over the native `composition`, reading
    /// the tile atlas `tiles` and each twin's HD page from `pages`, the
    /// world camera at `camera`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_tiles<'a>(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        placement: &HdPlacement<'_>,
        composition: &wgpu::Texture,
        tiles: &wgpu::TextureView,
        pages: &dyn Fn(usize) -> Option<&'a PictureHdPage>,
        camera: [f32; 2],
        scissor: [u32; 4],
    ) {
        if placement.scale <= 1.0 {
            return;
        }
        let terrain_placement = crate::render::hd_terrain::TerrainPlacement {
            origin: placement.origin,
            scale: placement.scale,
            mask: placement.mask.as_ref().map(|mask| HdMask {
                frame_origin: mask.frame_origin,
                frame_scale: mask.frame_scale,
                now: mask.now,
                before: mask.before,
            }),
        };
        for (page, draws) in &self.tiles {
            let Some(page) = pages(*page) else {
                continue;
            };
            // As `draw_passes::hd_terrain_pieces` places a tile.
            let draws: Vec<crate::render::hd_terrain::HdTerrainDraw> = draws
                .iter()
                .map(|draw| crate::render::hd_terrain::HdTerrainDraw {
                    origin: [
                        (draw.position[0] - camera[0] + 0.5).floor(),
                        (draw.position[1] - camera[1] + 0.5).floor(),
                    ],
                    size: draw.size,
                    atlas_origin: draw.atlas_origin,
                    hd_origin: draw.hd_origin,
                    hd_scale: [draw.scale, 0],
                })
                .collect();
            self.terrain.encode(
                device,
                encoder,
                view,
                composition,
                tiles,
                false,
                page.color_view(),
                &terrain_placement,
                scissor,
                &draws,
                false,
            );
        }
    }

    /// Draw this frame's runs into `view` at `placement`, inside `scissor`
    /// (target pixels), testing Z against `world_depth`, the world's own
    /// depth at one texel per world pixel, and ownership against
    /// `world_color`, the completed world's encoded colour there. `pages`
    /// gives each run's HD twin and the page the world drew from. `zshape` is
    /// the world runs' shape binding (`draw_native_object_pass`).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw<'a>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        placement: &HdPlacement<'_>,
        world_depth: &wgpu::TextureView,
        world_color: &wgpu::TextureView,
        pages: &dyn Fn(HdSource) -> Option<(&'a PictureHdPage, &'a BatchTexture)>,
        batch: &BatchRenderer,
        zshape: &wgpu::BindGroup,
        scissor: [u32; 4],
    ) {
        let (Some(buffer), Some(hd_uvs)) = (&self.instances, &self.hd_uvs) else {
            return;
        };
        if self.runs.is_empty() || placement.scale <= 1.0 {
            return;
        }
        let target = view.texture().size();
        let [x, y, width, height] = scissor;
        let (x, y) = (x.min(target.width), y.min(target.height));
        let width = width.min(target.width - x);
        let height = height.min(target.height - y);
        if width == 0 || height == 0 {
            return;
        }
        let (now, before, frame_origin, frame_scale, masked) = match &placement.mask {
            Some(mask) => (
                mask.now,
                mask.before,
                mask.frame_origin,
                mask.frame_scale,
                1,
            ),
            None => (&self.no_mask, &self.no_mask, [0.0; 2], 1.0, 0),
        };
        for depth in [HdDepth::None, HdDepth::Less, HdDepth::LessEqual] {
            let params = Params {
                origin: placement.origin,
                scale: placement.scale,
                policy: depth as u32,
                target_size: [target.width as f32, target.height as f32],
                frame_scale,
                masked,
                frame_origin,
                pad: [0.0; 2],
            };
            queue.write_buffer(
                &self.params,
                depth as u64 * self.params_stride,
                bytemuck::bytes_of(&params),
            );
        }
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Picture HD view"),
            layout: &self.view_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(world_depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.params,
                        offset: 0,
                        size: wgpu::BufferSize::new(PARAMS_SIZE),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(now),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(before),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(world_color),
                },
            ],
        });
        // One bind group per HD page this frame draws from.
        let mut page_groups: Vec<(HdSource, wgpu::BindGroup)> = Vec::new();
        for run in &self.runs {
            if page_groups.iter().any(|(page, _)| *page == run.page) {
                continue;
            }
            let Some((page, low)) = pages(run.page) else {
                continue;
            };
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Picture HD page"),
                layout: &self.page_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&page.color),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&page.indices),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&low.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&low.source_indices),
                    },
                ],
            });
            page_groups.push((run.page, group));
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Picture HD over the magnified world"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(3, zshape, &[]);
        pass.set_vertex_buffer(0, buffer.slice(..));
        pass.set_vertex_buffer(1, hd_uvs.slice(..));
        pass.set_scissor_rect(x, y, width, height);
        for run in &self.runs {
            let Some((_, group)) = page_groups.iter().find(|(page, _)| *page == run.page) else {
                continue;
            };
            let offset = run.depth as u64 * self.params_stride;
            // The camera the world draw bound: the same view, with the
            // tactical A plane or the neutral one.
            let camera = if run.neutral_a {
                batch.surface_camera_bind_group()
            } else {
                batch.camera_bind_group()
            };
            pass.set_bind_group(0, camera, &[]);
            pass.set_bind_group(1, group, &[]);
            pass.set_bind_group(2, &view_group, &[offset as u32]);
            pass.draw(0..6, run.start..run.start + run.count);
        }
    }
}
