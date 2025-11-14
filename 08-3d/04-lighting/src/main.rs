use cgmath::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// Vertex structure with position, normal, and color
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3], // Surface normal for lighting calculations
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Cube vertices with normals
// Each face needs separate vertices because normals differ per face
const VERTICES: &[Vertex] = &[
    // Front face (normal: +Z)
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        color: [0.8, 0.2, 0.2],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        color: [0.8, 0.2, 0.2],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        color: [0.8, 0.2, 0.2],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        color: [0.8, 0.2, 0.2],
    },
    // Back face (normal: -Z)
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        color: [0.2, 0.2, 0.8],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        color: [0.2, 0.2, 0.8],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        color: [0.2, 0.2, 0.8],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        color: [0.2, 0.2, 0.8],
    },
    // Right face (normal: +X)
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [1.0, 0.0, 0.0],
        color: [0.2, 0.8, 0.2],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [1.0, 0.0, 0.0],
        color: [0.2, 0.8, 0.2],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [1.0, 0.0, 0.0],
        color: [0.2, 0.8, 0.2],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [1.0, 0.0, 0.0],
        color: [0.2, 0.8, 0.2],
    },
    // Left face (normal: -X)
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [-1.0, 0.0, 0.0],
        color: [0.8, 0.8, 0.2],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
        color: [0.8, 0.8, 0.2],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
        color: [0.8, 0.8, 0.2],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [-1.0, 0.0, 0.0],
        color: [0.8, 0.8, 0.2],
    },
    // Top face (normal: +Y)
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
        color: [0.8, 0.2, 0.8],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
        color: [0.8, 0.2, 0.8],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [0.0, 1.0, 0.0],
        color: [0.8, 0.2, 0.8],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [0.0, 1.0, 0.0],
        color: [0.8, 0.2, 0.8],
    },
    // Bottom face (normal: -Y)
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [0.0, -1.0, 0.0],
        color: [0.2, 0.8, 0.8],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [0.0, -1.0, 0.0],
        color: [0.2, 0.8, 0.8],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [0.0, -1.0, 0.0],
        color: [0.2, 0.8, 0.8],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [0.0, -1.0, 0.0],
        color: [0.2, 0.8, 0.8],
    },
];

// Indices for the cube
const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // Front
    4, 5, 6, 6, 7, 4, // Back
    8, 9, 10, 10, 11, 8, // Right
    12, 13, 14, 14, 15, 12, // Left
    16, 17, 18, 18, 19, 16, // Top
    20, 21, 22, 22, 23, 20, // Bottom
];

/// Uniform buffer containing transformation matrices and lighting parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    normal_matrix: [[f32; 4]; 4], // For transforming normals
    // Light properties
    light_position: [f32; 3],
    _padding1: f32, // Alignment
    light_color: [f32; 3],
    _padding2: f32,
    // Camera position for specular highlights
    camera_position: [f32; 3],
    _padding3: f32,
    // Material properties
    ambient_strength: f32,
    diffuse_strength: f32,
    specular_strength: f32,
    shininess: f32,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            model: cgmath::Matrix4::identity().into(),
            view: cgmath::Matrix4::identity().into(),
            projection: cgmath::Matrix4::identity().into(),
            normal_matrix: cgmath::Matrix4::identity().into(),
            light_position: [3.0, 4.0, 5.0],
            _padding1: 0.0,
            light_color: [1.0, 1.0, 1.0],
            _padding2: 0.0,
            camera_position: [0.0, 2.0, 5.0],
            _padding3: 0.0,
            ambient_strength: 0.1,
            diffuse_strength: 1.0,
            specular_strength: 0.5,
            shininess: 32.0,
        }
    }

    fn update_matrices(
        &mut self,
        model: cgmath::Matrix4<f32>,
        view: cgmath::Matrix4<f32>,
        projection: cgmath::Matrix4<f32>,
    ) {
        self.model = model.into();
        self.view = view.into();
        self.projection = projection.into();

        // Normal matrix is the inverse transpose of the model matrix
        // This ensures normals are transformed correctly with non-uniform scaling
        if let Some(inv_model) = model.invert() {
            self.normal_matrix = inv_model.transpose().into();
        }
    }

    fn update_camera_position(&mut self, position: cgmath::Point3<f32>) {
        self.camera_position = position.into();
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    rotation: f32,
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let uniforms = Uniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
            uniforms,
            depth_texture,
            depth_view,
            rotation: 0.0,
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
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            self.depth_view = self
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.rotation += 0.01;

        let model = cgmath::Matrix4::from_angle_x(cgmath::Rad(self.rotation * 0.5))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(self.rotation));

        let camera_pos = cgmath::Point3::new(0.0, 2.0, 5.0);
        let view = cgmath::Matrix4::look_at_rh(
            camera_pos,
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
        );

        let aspect_ratio = self.config.width as f32 / self.config.height as f32;
        let projection = cgmath::perspective(cgmath::Deg(45.0), aspect_ratio, 0.1, 100.0);

        self.uniforms.update_matrices(model, view, projection);
        self.uniforms.update_camera_position(camera_pos);

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
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

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.1,
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
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
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
        .with_title("wgpu Lighting (Phong)")
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
