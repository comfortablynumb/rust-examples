use cgmath::prelude::*;
use winit::{
    dpi::PhysicalPosition,
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

// Cube geometry
fn create_cube() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        // Front
        Vertex {
            position: [-0.5, -0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
        },
        // Back
        Vertex {
            position: [0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
        },
        // Right
        Vertex {
            position: [0.5, -0.5, 0.5],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            normal: [1.0, 0.0, 0.0],
        },
        // Left
        Vertex {
            position: [-0.5, -0.5, -0.5],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            normal: [-1.0, 0.0, 0.0],
        },
        // Top
        Vertex {
            position: [-0.5, 0.5, 0.5],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            normal: [0.0, 1.0, 0.0],
        },
        // Bottom
        Vertex {
            position: [-0.5, -0.5, -0.5],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            normal: [0.0, -1.0, 0.0],
        },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ];

    (vertices, indices)
}

// Sphere geometry (icosphere approximation)
fn create_sphere(subdivisions: u32) -> (Vec<Vertex>, Vec<u16>) {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

    let mut vertices = vec![
        Vertex {
            position: [-1.0, t, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, t, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, -t, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, -t, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, -1.0, t],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 1.0, t],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, -1.0, -t],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 1.0, -t],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [t, 0.0, -1.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [t, 0.0, 1.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [-t, 0.0, -1.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [-t, 0.0, 1.0],
            normal: [0.0, 0.0, 0.0],
        },
    ];

    // Normalize positions to create unit sphere and set normals
    for vertex in &mut vertices {
        let len =
            (vertex.position[0].powi(2) + vertex.position[1].powi(2) + vertex.position[2].powi(2))
                .sqrt();
        vertex.position[0] /= len;
        vertex.position[1] /= len;
        vertex.position[2] /= len;
        vertex.normal = vertex.position;
    }

    let indices = vec![
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7,
        1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9,
        8, 1,
    ];

    (vertices, indices)
}

/// Material properties for an object
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Material {
    color: [f32; 3],
    _padding1: f32,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

/// Instance data for rendering multiple objects
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceData {
    model: [[f32; 4]; 4],
    normal_matrix: [[f32; 4]; 4],
}

/// Scene object with transform and material
struct SceneObject {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Vector3<f32>,
    scale: cgmath::Vector3<f32>,
    material: Material,
    mesh_type: MeshType,
}

enum MeshType {
    Cube,
    Sphere,
}

impl SceneObject {
    fn model_matrix(&self) -> cgmath::Matrix4<f32> {
        let translation = cgmath::Matrix4::from_translation(self.position);
        let rotation = cgmath::Matrix4::from_angle_x(cgmath::Rad(self.rotation.x))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(self.rotation.y))
            * cgmath::Matrix4::from_angle_z(cgmath::Rad(self.rotation.z));
        let scale =
            cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        translation * rotation * scale
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    light_position: [f32; 3],
    _padding1: f32,
    light_color: [f32; 3],
    _padding2: f32,
    camera_position: [f32; 3],
    _padding3: f32,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            view: cgmath::Matrix4::identity().into(),
            projection: cgmath::Matrix4::identity().into(),
            light_position: [5.0, 5.0, 5.0],
            _padding1: 0.0,
            light_color: [1.0, 1.0, 1.0],
            _padding2: 0.0,
            camera_position: [0.0, 2.0, 10.0],
            _padding3: 0.0,
        }
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

    // Geometry
    cube_vertex_buffer: wgpu::Buffer,
    cube_index_buffer: wgpu::Buffer,
    cube_num_indices: u32,
    sphere_vertex_buffer: wgpu::Buffer,
    sphere_index_buffer: wgpu::Buffer,
    sphere_num_indices: u32,

    // Uniforms
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,

    // Materials
    material_buffer: wgpu::Buffer,
    material_bind_group: wgpu::BindGroup,

    // Depth
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    // Scene
    objects: Vec<SceneObject>,
    camera_angle: f32,
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

        // Create geometries
        let (cube_vertices, cube_indices) = create_cube();
        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let cube_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&cube_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let cube_num_indices = cube_indices.len() as u32;

        let (sphere_vertices, sphere_indices) = create_sphere(2);
        let sphere_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&sphere_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sphere_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&sphere_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let sphere_num_indices = sphere_indices.len() as u32;

        // Create scene objects
        let objects = vec![
            // Ground plane (scaled cube)
            SceneObject {
                position: cgmath::Vector3::new(0.0, -1.0, 0.0),
                rotation: cgmath::Vector3::zero(),
                scale: cgmath::Vector3::new(10.0, 0.1, 10.0),
                material: Material {
                    color: [0.3, 0.3, 0.3],
                    _padding1: 0.0,
                    ambient: 0.2,
                    diffuse: 0.8,
                    specular: 0.1,
                    shininess: 4.0,
                },
                mesh_type: MeshType::Cube,
            },
            // Red cube
            SceneObject {
                position: cgmath::Vector3::new(-2.0, 0.5, 0.0),
                rotation: cgmath::Vector3::new(0.3, 0.5, 0.0),
                scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
                material: Material {
                    color: [0.8, 0.2, 0.2],
                    _padding1: 0.0,
                    ambient: 0.1,
                    diffuse: 1.0,
                    specular: 0.5,
                    shininess: 32.0,
                },
                mesh_type: MeshType::Cube,
            },
            // Green sphere
            SceneObject {
                position: cgmath::Vector3::new(0.0, 0.5, 0.0),
                rotation: cgmath::Vector3::zero(),
                scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
                material: Material {
                    color: [0.2, 0.8, 0.2],
                    _padding1: 0.0,
                    ambient: 0.1,
                    diffuse: 1.0,
                    specular: 0.8,
                    shininess: 64.0,
                },
                mesh_type: MeshType::Sphere,
            },
            // Blue cube
            SceneObject {
                position: cgmath::Vector3::new(2.0, 0.5, 0.0),
                rotation: cgmath::Vector3::new(0.0, 0.8, 0.3),
                scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
                material: Material {
                    color: [0.2, 0.2, 0.8],
                    _padding1: 0.0,
                    ambient: 0.1,
                    diffuse: 1.0,
                    specular: 0.5,
                    shininess: 32.0,
                },
                mesh_type: MeshType::Cube,
            },
        ];

        // Create uniforms
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

        // Create material buffer
        let material = objects[0].material;
        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::cast_slice(&[material]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("material_bind_group_layout"),
            });

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_buffer.as_entire_binding(),
            }],
            label: Some("material_bind_group"),
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout, &material_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..128, // Two 4x4 matrices
                }],
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
            render_pipeline,
            cube_vertex_buffer,
            cube_index_buffer,
            cube_num_indices,
            sphere_vertex_buffer,
            sphere_index_buffer,
            sphere_num_indices,
            uniform_buffer,
            uniform_bind_group,
            uniforms,
            material_buffer,
            material_bind_group,
            depth_texture,
            depth_view,
            objects,
            camera_angle: 0.0,
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
        self.camera_angle += 0.01;

        // Rotate camera around scene
        let camera_distance = 10.0;
        let camera_pos = cgmath::Point3::new(
            camera_angle.cos() * camera_distance,
            3.0,
            camera_angle.sin() * camera_distance,
        );

        let view = cgmath::Matrix4::look_at_rh(
            camera_pos,
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
        );

        let aspect = self.config.width as f32 / self.config.height as f32;
        let projection = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);

        self.uniforms.view = view.into();
        self.uniforms.projection = projection.into();
        self.uniforms.camera_position = camera_pos.into();

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        // Animate objects
        self.objects[1].rotation.y += 0.01;
        self.objects[3].rotation.x += 0.015;
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

            // Render each object
            for object in &self.objects {
                // Update material
                self.queue.write_buffer(
                    &self.material_buffer,
                    0,
                    bytemuck::cast_slice(&[object.material]),
                );
                render_pass.set_bind_group(1, &self.material_bind_group, &[]);

                // Calculate matrices
                let model = object.model_matrix();
                let normal_matrix = if let Some(inv) = model.invert() {
                    inv.transpose()
                } else {
                    cgmath::Matrix4::identity()
                };

                // Push constants for per-object matrices
                let instance_data = InstanceData {
                    model: model.into(),
                    normal_matrix: normal_matrix.into(),
                };
                render_pass.set_push_constants(
                    wgpu::ShaderStages::VERTEX,
                    0,
                    bytemuck::cast_slice(&[instance_data]),
                );

                // Draw the appropriate mesh
                match object.mesh_type {
                    MeshType::Cube => {
                        render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            self.cube_index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(0..self.cube_num_indices, 0, 0..1);
                    }
                    MeshType::Sphere => {
                        render_pass.set_vertex_buffer(0, self.sphere_vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            self.sphere_index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(0..self.sphere_num_indices, 0, 0..1);
                    }
                }
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
        .with_title("wgpu Scene with Multiple Objects")
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
