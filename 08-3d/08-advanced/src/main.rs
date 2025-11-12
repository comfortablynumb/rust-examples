use cgmath::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

fn create_cube() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        // Front
        Vertex { position: [-0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [0.5, 0.5, 0.5], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-0.5, 0.5, 0.5], normal: [0.0, 0.0, 1.0] },
        // Back
        Vertex { position: [0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-0.5, 0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [0.5, 0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        // Right
        Vertex { position: [0.5, -0.5, 0.5], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [0.5, 0.5, -0.5], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], normal: [1.0, 0.0, 0.0] },
        // Left
        Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.5], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, -0.5], normal: [-1.0, 0.0, 0.0] },
        // Top
        Vertex { position: [-0.5, 0.5, 0.5], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [0.5, 0.5, -0.5], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-0.5, 0.5, -0.5], normal: [0.0, 1.0, 0.0] },
        // Bottom
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.5], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], normal: [0.0, -1.0, 0.0] },
    ];
    let indices = vec![
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ];
    (vertices, indices)
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ShadowUniforms {
    light_view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct RenderUniforms {
    view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    normal_matrix: [[f32; 4]; 4],
    light_view_proj: [[f32; 4]; 4],
    light_position: [f32; 3],
    _padding1: f32,
    light_color: [f32; 3],
    _padding2: f32,
    camera_position: [f32; 3],
    _padding3: f32,
    object_color: [f32; 3],
    _padding4: f32,
}

const SHADOW_MAP_SIZE: u32 = 2048;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,

    // Shadow pass
    shadow_pipeline: wgpu::RenderPipeline,
    shadow_texture: wgpu::Texture,
    shadow_view: wgpu::TextureView,
    shadow_sampler: wgpu::Sampler,
    shadow_bind_group: wgpu::BindGroup,
    shadow_uniform_buffer: wgpu::Buffer,

    // Render pass
    render_pipeline: wgpu::RenderPipeline,
    render_bind_group: wgpu::BindGroup,
    render_uniform_buffer: wgpu::Buffer,

    // Geometry
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    // Depth
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    // Animation
    time: f32,
    light_angle: f32,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Create shadow map texture
        let shadow_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map"),
            size: wgpu::Extent3d {
                width: SHADOW_MAP_SIZE,
                height: SHADOW_MAP_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let shadow_view = shadow_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create geometry
        let (vertices, indices) = create_cube();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        // Shadow uniform buffer
        let shadow_uniforms = ShadowUniforms {
            light_view_proj: cgmath::Matrix4::identity().into(),
            model: cgmath::Matrix4::identity().into(),
        };
        let shadow_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Uniform Buffer"),
            contents: bytemuck::cast_slice(&[shadow_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Render uniform buffer
        let render_uniforms = RenderUniforms {
            view_proj: cgmath::Matrix4::identity().into(),
            model: cgmath::Matrix4::identity().into(),
            normal_matrix: cgmath::Matrix4::identity().into(),
            light_view_proj: cgmath::Matrix4::identity().into(),
            light_position: [5.0, 5.0, 5.0],
            _padding1: 0.0,
            light_color: [1.0, 1.0, 0.9],
            _padding2: 0.0,
            camera_position: [0.0, 5.0, 10.0],
            _padding3: 0.0,
            object_color: [0.8, 0.2, 0.2],
            _padding4: 0.0,
        };
        let render_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Render Uniform Buffer"),
            contents: bytemuck::cast_slice(&[render_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Shadow bind group layout
        let shadow_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("shadow_bind_group_layout"),
            });

        let shadow_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &shadow_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: shadow_uniform_buffer.as_entire_binding(),
            }],
            label: Some("shadow_bind_group"),
        });

        // Render bind group layout
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                        count: None,
                    },
                ],
                label: Some("render_bind_group_layout"),
            });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: render_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&shadow_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&shadow_sampler),
                },
            ],
            label: Some("render_bind_group"),
        });

        // Load shaders
        let shadow_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shadow.wgsl").into()),
        });

        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("render.wgsl").into()),
        });

        // Create shadow pipeline
        let shadow_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shadow Pipeline Layout"),
                bind_group_layouts: &[&shadow_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: Some(&shadow_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shadow_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: None, // Depth-only pass
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            shadow_pipeline,
            shadow_texture,
            shadow_view,
            shadow_sampler,
            shadow_bind_group,
            shadow_uniform_buffer,
            render_pipeline,
            render_bind_group,
            render_uniform_buffer,
            vertex_buffer,
            index_buffer,
            num_indices,
            depth_texture,
            depth_view,
            time: 0.0,
            light_angle: 0.0,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: new_size.width,
                    height: new_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.depth_view = self
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.time += 0.016;
        self.light_angle += 0.01;

        // Light position orbits around scene
        let light_pos = cgmath::Point3::new(
            self.light_angle.cos() * 8.0,
            5.0,
            self.light_angle.sin() * 8.0,
        );

        // Light view-projection for shadow map
        let light_view = cgmath::Matrix4::look_at_rh(
            light_pos,
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
        );
        let light_projection = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, 1.0, 20.0);
        let light_view_proj = light_projection * light_view;

        // Camera view-projection
        let camera_pos = cgmath::Point3::new(0.0, 5.0, 10.0);
        let view = cgmath::Matrix4::look_at_rh(
            camera_pos,
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
        );
        let aspect = self.config.width as f32 / self.config.height as f32;
        let projection = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);
        let view_proj = projection * view;

        // Update shadow uniforms for each object
        let objects = vec![
            (cgmath::Vector3::new(0.0, -1.0, 0.0), cgmath::Vector3::new(10.0, 0.1, 10.0)),
            (cgmath::Vector3::new(-2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
            (cgmath::Vector3::new(0.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
            (cgmath::Vector3::new(2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
        ];

        // Store matrices for render pass
        let render_uniforms = RenderUniforms {
            view_proj: view_proj.into(),
            model: cgmath::Matrix4::identity().into(),
            normal_matrix: cgmath::Matrix4::identity().into(),
            light_view_proj: light_view_proj.into(),
            light_position: light_pos.into(),
            _padding1: 0.0,
            light_color: [1.0, 1.0, 0.9],
            _padding2: 0.0,
            camera_position: camera_pos.into(),
            _padding3: 0.0,
            object_color: [0.8, 0.2, 0.2],
            _padding4: 0.0,
        };
        self.queue.write_buffer(
            &self.render_uniform_buffer,
            0,
            bytemuck::cast_slice(&[render_uniforms]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Shadow pass
        {
            let mut shadow_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Shadow Pass"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            shadow_pass.set_pipeline(&self.shadow_pipeline);
            shadow_pass.set_bind_group(0, &self.shadow_bind_group, &[]);
            shadow_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            shadow_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw all objects to shadow map
            let objects = vec![
                (cgmath::Vector3::new(0.0, -1.0, 0.0), cgmath::Vector3::new(10.0, 0.1, 10.0)),
                (cgmath::Vector3::new(-2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
                (cgmath::Vector3::new(0.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
                (cgmath::Vector3::new(2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0)),
            ];

            for (pos, scale) in objects {
                let model = cgmath::Matrix4::from_translation(pos)
                    * cgmath::Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

                // Update shadow uniforms
                let light_pos = cgmath::Point3::new(
                    self.light_angle.cos() * 8.0,
                    5.0,
                    self.light_angle.sin() * 8.0,
                );
                let light_view = cgmath::Matrix4::look_at_rh(
                    light_pos,
                    cgmath::Point3::new(0.0, 0.0, 0.0),
                    cgmath::Vector3::unit_y(),
                );
                let light_projection = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, 1.0, 20.0);
                let light_view_proj = light_projection * light_view;

                let shadow_uniforms = ShadowUniforms {
                    light_view_proj: light_view_proj.into(),
                    model: model.into(),
                };
                self.queue.write_buffer(
                    &self.shadow_uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[shadow_uniforms]),
                );

                shadow_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }

        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw all objects with shadows
            let objects = vec![
                (cgmath::Vector3::new(0.0, -1.0, 0.0), cgmath::Vector3::new(10.0, 0.1, 10.0), [0.3, 0.3, 0.3]),
                (cgmath::Vector3::new(-2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0), [0.8, 0.2, 0.2]),
                (cgmath::Vector3::new(0.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0), [0.2, 0.8, 0.2]),
                (cgmath::Vector3::new(2.0, 0.5, 0.0), cgmath::Vector3::new(1.0, 1.0, 1.0), [0.2, 0.2, 0.8]),
            ];

            for (pos, scale, color) in objects {
                let model = cgmath::Matrix4::from_translation(pos)
                    * cgmath::Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
                let normal_matrix = if let Some(inv) = model.invert() {
                    inv.transpose()
                } else {
                    cgmath::Matrix4::identity()
                };

                let light_pos = cgmath::Point3::new(
                    self.light_angle.cos() * 8.0,
                    5.0,
                    self.light_angle.sin() * 8.0,
                );
                let light_view = cgmath::Matrix4::look_at_rh(
                    light_pos,
                    cgmath::Point3::new(0.0, 0.0, 0.0),
                    cgmath::Vector3::unit_y(),
                );
                let light_projection = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, 1.0, 20.0);
                let light_view_proj = light_projection * light_view;

                let camera_pos = cgmath::Point3::new(0.0, 5.0, 10.0);
                let view = cgmath::Matrix4::look_at_rh(
                    camera_pos,
                    cgmath::Point3::new(0.0, 0.0, 0.0),
                    cgmath::Vector3::unit_y(),
                );
                let aspect = self.config.width as f32 / self.config.height as f32;
                let projection = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);
                let view_proj = projection * view;

                let render_uniforms = RenderUniforms {
                    view_proj: view_proj.into(),
                    model: model.into(),
                    normal_matrix: normal_matrix.into(),
                    light_view_proj: light_view_proj.into(),
                    light_position: light_pos.into(),
                    _padding1: 0.0,
                    light_color: [1.0, 1.0, 0.9],
                    _padding2: 0.0,
                    camera_position: camera_pos.into(),
                    _padding3: 0.0,
                    object_color: color,
                    _padding4: 0.0,
                };
                self.queue.write_buffer(
                    &self.render_uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[render_uniforms]),
                );

                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu Advanced - Shadow Mapping")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.window().request_redraw();
        }
        _ => {}
    });
}

use wgpu::util::DeviceExt;
