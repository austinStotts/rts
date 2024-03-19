use iced_wgpu::wgpu::{self, util::DeviceExt};
use iced_winit::core::Color;
use image;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    texcoord: [f32; 2], 
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Parameters {
    // radius1: f32,
    sigma1: f32,
    // radius2: f32,
    // sigma2: f32,
    tau: f32,
    gfact: f32,
    epsilon: f32,
    num_gvf_iterations: i32,
    enable_xdog: u32,
}

struct RenderingPipeline {
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    parameters_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
}

pub struct Scene {
    pipeline: RenderingPipeline,
}

impl Scene {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        queue: &wgpu::Queue
    ) -> Scene {
        let rendering_pipeline = build_pipeline(device, texture_format, queue);

        Scene { pipeline: rendering_pipeline }
    }

    pub fn clear<'a>(
        target: &'a wgpu::TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
        background_color: Color,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear({
                        let [r, g, b, a] = background_color.into_linear();

                        wgpu::Color {
                            r: r as f64,
                            g: g as f64,
                            b: b as f64,
                            a: a as f64,
                        }
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline.render_pipeline);
        render_pass.set_bind_group(0, &self.pipeline.texture_bind_group, &[]);
        render_pass.set_bind_group(1, &self.pipeline.parameters_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}

fn build_pipeline(
    device: &wgpu::Device,
    texture_format: wgpu::TextureFormat,
    queue: &wgpu::Queue,
) -> RenderingPipeline {
    let (vert_module, frag_module) = (
        device.create_shader_module(wgpu::include_wgsl!("../shaders/flow-based-xdog/vertex.wgsl")),
        device.create_shader_module(wgpu::include_wgsl!("../shaders/flow-based-xdog/fragment.wgsl")),
    );

    let vertex_data = [
        Vertex { position: [-1.0, -1.0], texcoord: [0.0, 1.0] }, // Bottom-left
        Vertex { position: [-1.0, 1.0], texcoord: [0.0, 0.0] },  // Top-left
        Vertex { position: [1.0, 1.0], texcoord: [1.0, 0.0] },   // Top-right
        Vertex { position: [1.0, 1.0], texcoord: [1.0, 0.0] },   // Top-right (repeated)
        Vertex { position: [1.0, -1.0], texcoord: [1.0, 1.0] },  // Bottom-right
        Vertex { position: [-1.0, -1.0], texcoord: [0.0, 1.0] }, // Bottom-left (repeated)
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: unsafe {
            std::slice::from_raw_parts(
                vertex_data.as_ptr() as *const u8,
                vertex_data.len() * std::mem::size_of::<Vertex>(),
            )
        },
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    let img = image::load_from_memory(include_bytes!("../images/cat.png")).unwrap();
    let img_ = img.to_rgba8();
    let (mut width, mut height) = img_.dimensions();
    let image = img.resize(width, height, image::imageops::FilterType::Gaussian).to_rgba8();
    // window.set_inner_size(LogicalSize::new(width, height));
    let image_data = image.into_vec();


    let image_texture = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
        }
    );

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &image_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All
        },
        &image_data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1
        },
    );


    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });
    

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&image_texture.create_view(&wgpu::TextureViewDescriptor::default())),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                })),
            },
        ],
    });


    let params = Parameters { 
        // radius1: 1.0,
        sigma1: 4.75,
        // radius2: 4.0,
        // sigma2: 6.0,
        tau: 0.075,
        gfact: 8.0,
        epsilon: 0.0001,
        num_gvf_iterations: 30,
        enable_xdog: 1,
    };


    let params_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Parameter buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }
    );

    let params_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("paramerters bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None 
                },
                count: None
            }
        ]
    });

    let parameters_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("parameters bind group"),
        layout: &params_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: params_buffer.as_entire_binding(),
            }
        ]
    });



    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        // bind_group_layouts: &[&texture_bind_group_layout, &palette_bind_group_layout],
        bind_group_layouts: &[&texture_bind_group_layout, &params_bind_group_layout,],
        push_constant_ranges: &[],
    });


    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vert_module,
            entry_point: "vert_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_module,
            entry_point: "frag_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    return RenderingPipeline {
        render_pipeline,
        texture_bind_group,
        parameters_bind_group,
        vertex_buffer,
    };


    // device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    //     label: None,
    //     layout: Some(&pipeline_layout),
    //     vertex: wgpu::VertexState {
    //         module: &vs_module,
    //         entry_point: "main",
    //         buffers: &[],
    //     },
    //     fragment: Some(wgpu::FragmentState {
    //         module: &fs_module,
    //         entry_point: "main",
    //         targets: &[Some(wgpu::ColorTargetState {
    //             format: texture_format,
    //             blend: Some(wgpu::BlendState {
    //                 color: wgpu::BlendComponent::REPLACE,
    //                 alpha: wgpu::BlendComponent::REPLACE,
    //             }),
    //             write_mask: wgpu::ColorWrites::ALL,
    //         })],
    //     }),
    //     primitive: wgpu::PrimitiveState {
    //         topology: wgpu::PrimitiveTopology::TriangleList,
    //         front_face: wgpu::FrontFace::Ccw,
    //         ..Default::default()
    //     },
    //     depth_stencil: None,
    //     multisample: wgpu::MultisampleState {
    //         count: 1,
    //         mask: !0,
    //         alpha_to_coverage_enabled: false,
    //     },
    //     multiview: None,
    // })
}