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
    color: [f32; 3],
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
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.8, 0.2, 0.2] },
    Vertex { position: [1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.8, 0.2, 0.2] },
    Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.8, 0.2, 0.2] },
    Vertex { position: [-1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.8, 0.2, 0.2] },
    // Back face
    Vertex { position: [1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.2, 0.2, 0.8] },
    Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.2, 0.2, 0.8] },
    Vertex { position: [-1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.2, 0.2, 0.8] },
    Vertex { position: [1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.2, 0.2, 0.8] },
    // Right face
    Vertex { position: [1.0, -1.0, 1.0], normal: [1.0, 0.0, 0.0], color: [0.2, 0.8, 0.2] },
    Vertex { position: [1.0, -1.0, -1.0], normal: [1.0, 0.0, 0.0], color: [0.2, 0.8, 0.2] },
    Vertex { position: [1.0, 1.0, -1.0], normal: [1.0, 0.0, 0.0], color: [0.2, 0.8, 0.2] },
    Vertex { position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], color: [0.2, 0.8, 0.2] },
    // Left face
    Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0, 0.0, 0.0], color: [0.8, 0.8, 0.2] },
    Vertex { position: [-1.0, -1.0, 1.0], normal: [-1.0, 0.0, 0.0], color: [0.8, 0.8, 0.2] },
    Vertex { position: [-1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0], color: [0.8, 0.8, 0.2] },
    Vertex { position: [-1.0, 1.0, -1.0], normal: [-1.0, 0.0, 0.0], color: [0.8, 0.8, 0.2] },
    // Top face
    Vertex { position: [-1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], color: [0.8, 0.2, 0.8] },
    Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], color: [0.8, 0.2, 0.8] },
    Vertex { position: [1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0], color: [0.8, 0.2, 0.8] },
    Vertex { position: [-1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0], color: [0.8, 0.2, 0.8] },
    // Bottom face
    Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], color: [0.2, 0.8, 0.8] },
    Vertex { position: [1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], color: [0.2, 0.8, 0.8] },
    Vertex { position: [1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0], color: [0.2, 0.8, 0.8] },
    Vertex { position: [-1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0], color: [0.2, 0.8, 0.8] },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4,
    8, 9, 10, 10, 11, 8,
    12, 13, 14, 14, 15, 12,
    16, 17, 18, 18, 19, 16,
    20, 21, 22, 22, 23, 20,
];

/// Camera controller for first-person style camera movement
struct Camera {
    /// Camera position in world space
    position: cgmath::Point3<f32>,
    /// Yaw angle (rotation around Y axis) in radians
    yaw: f32,
    /// Pitch angle (rotation around X axis) in radians
    pitch: f32,
    /// Field of view in degrees
    fov: f32,
    /// Aspect ratio (width / height)
    aspect: f32,
    /// Near clipping plane
    near: f32,
    /// Far clipping plane
    far: f32,
}

impl Camera {
    fn new(aspect: f32) -> Self {
        Self {
            position: cgmath::Point3::new(0.0, 2.0, 5.0),
            yaw: -std::f32::consts::PI / 2.0, // Looking towards -Z
            pitch: 0.0,
            fov: 45.0,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    /// Get the forward direction vector
    fn forward(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    /// Get the right direction vector
    fn right(&self) -> cgmath::Vector3<f32> {
        self.forward().cross(cgmath::Vector3::unit_y()).normalize()
    }

    /// Get the up direction vector
    fn up(&self) -> cgmath::Vector3<f32> {
        self.right().cross(self.forward()).normalize()
    }

    /// Build the view matrix
    fn build_view_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_at_rh(
            self.position,
            self.position + self.forward(),
            cgmath::Vector3::unit_y(),
        )
    }

    /// Build the projection matrix
    fn build_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::perspective(cgmath::Deg(self.fov), self.aspect, self.near, self.far)
    }

    /// Update aspect ratio (called on window resize)
    fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

/// Camera controller for handling input
struct CameraController {
    /// Movement speed
    speed: f32,
    /// Mouse sensitivity
    sensitivity: f32,
    /// Movement flags
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    /// Mouse movement
    mouse_dx: f32,
    mouse_dy: f32,
}

impl CameraController {
    fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
        }
    }

    /// Process keyboard input
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let pressed = state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.forward = pressed;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.backward = pressed;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.left = pressed;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.right = pressed;
                true
            }
            VirtualKeyCode::Space => {
                self.up = pressed;
                true
            }
            VirtualKeyCode::LShift => {
                self.down = pressed;
                true
            }
            _ => false,
        }
    }

    /// Process mouse movement
    fn process_mouse(&mut self, dx: f64, dy: f64) {
        self.mouse_dx = dx as f32;
        self.mouse_dy = dy as f32;
    }

    /// Update camera based on inputs
    fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        // Apply mouse movement to camera rotation
        camera.yaw += self.mouse_dx * self.sensitivity * dt;
        camera.pitch -= self.mouse_dy * self.sensitivity * dt;

        // Clamp pitch to prevent camera flipping
        camera.pitch = camera
            .pitch
            .max(-std::f32::consts::PI / 2.0 + 0.01)
            .min(std::f32::consts::PI / 2.0 - 0.01);

        // Reset mouse delta
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;

        // Calculate movement
        let mut velocity = cgmath::Vector3::zero();

        if self.forward {
            velocity += camera.forward();
        }
        if self.backward {
            velocity -= camera.forward();
        }
        if self.right {
            velocity += camera.right();
        }
        if self.left {
            velocity -= camera.right();
        }
        if self.up {
            velocity += cgmath::Vector3::unit_y();
        }
        if self.down {
            velocity -= cgmath::Vector3::unit_y();
        }

        // Normalize to prevent faster diagonal movement
        if velocity.magnitude() > 0.0 {
            velocity = velocity.normalize();
        }

        // Apply movement
        camera.position += velocity * self.speed * dt;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    normal_matrix: [[f32; 4]; 4],
    light_position: [f32; 3],
    _padding1: f32,
    light_color: [f32; 3],
    _padding2: f32,
    camera_position: [f32; 3],
    _padding3: f32,
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
            light_position: [5.0, 5.0, 5.0],
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
    camera: Camera,
    camera_controller: CameraController,
    mouse_pressed: bool,
    last_mouse_pos: PhysicalPosition<f64>,
    last_frame_time: std::time::Instant,
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

        let camera = Camera::new(config.width as f32 / config.height as f32);
        let camera_controller = CameraController::new(5.0, 1.0);

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
            camera,
            camera_controller,
            mouse_pressed: false,
            last_mouse_pos: PhysicalPosition::new(0.0, 0.0),
            last_frame_time: std::time::Instant::now(),
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

            self.camera
                .update_aspect(new_size.width as f32 / new_size.height as f32);

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

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed {
                    let dx = position.x - self.last_mouse_pos.x;
                    let dy = position.y - self.last_mouse_pos.y;
                    self.camera_controller.process_mouse(dx, dy);
                }
                self.last_mouse_pos = *position;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        // Update camera with controller
        self.camera_controller.update_camera(&mut self.camera, dt);

        // Create matrices
        let model = cgmath::Matrix4::identity();
        let view = self.camera.build_view_matrix();
        let projection = self.camera.build_projection_matrix();

        self.uniforms.update_matrices(model, view, projection);
        self.uniforms.update_camera_position(self.camera.position);

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
        .with_title("wgpu Camera Controls")
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
