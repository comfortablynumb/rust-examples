use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};

/// Vertex structure with interleaved position and color data
/// This demonstrates how to pack multiple attributes into a single struct
/// bytemuck::Pod and bytemuck::Zeroable allow us to safely cast this to bytes
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3], // x, y, z coordinates
    color: [f32; 3],    // r, g, b color values
}

impl Vertex {
    /// Describes the layout of vertex data to wgpu
    /// This tells the GPU how to interpret the raw bytes in the vertex buffer
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // The stride is the byte size of one vertex
            // This tells the GPU how many bytes to skip to get to the next vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            // How often to move to the next vertex
            // Vertex means move to the next vertex for each vertex
            // Instance would mean move to the next vertex for each instance
            step_mode: wgpu::VertexStepMode::Vertex,

            // The individual attributes (position, color, etc.)
            attributes: &[
                // Position attribute at location 0
                wgpu::VertexAttribute {
                    offset: 0, // Starts at byte 0
                    shader_location: 0, // Matches @location(0) in vertex shader
                    format: wgpu::VertexFormat::Float32x3, // 3 floats (x, y, z)
                },
                // Color attribute at location 1
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // Starts after position (12 bytes)
                    shader_location: 1, // Matches @location(1) in vertex shader
                    format: wgpu::VertexFormat::Float32x3, // 3 floats (r, g, b)
                },
            ],
        }
    }
}

// Define the vertices for a square (quad)
// We use a coordinate system where:
// - Center is (0, 0)
// - Right is +X, Up is +Y
// - Each vertex has both position and color
const VERTICES: &[Vertex] = &[
    // Top-left - Red
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    // Top-right - Green
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    // Bottom-right - Blue
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    // Bottom-left - Yellow
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 1.0, 0.0],
    },
];

// Index buffer - defines which vertices form which triangles
// This allows us to reuse vertices instead of duplicating them
//
// Without indices: 6 vertices needed (2 triangles * 3 vertices each)
// With indices: 4 vertices + 6 indices (more efficient!)
//
// Square layout:
// 0---1
// |  /|
// | / |
// |/  |
// 3---2
//
// Triangle 1: vertices 0, 1, 2 (top-left, top-right, bottom-right)
// Triangle 2: vertices 0, 2, 3 (top-left, bottom-right, bottom-left)
const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    0, 2, 3, // Second triangle
];

/// Represents the GPU state and rendering pipeline
struct State<'a> {
    window: Arc<Window>,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    // GPU buffers for vertex and index data
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl<'a> State<'a> {
    async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        // Instance, surface, adapter, device, queue setup (same as triangle example)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create the vertex buffer
        // This buffer will live on the GPU and contain our vertex data
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES), // Convert vertices to bytes
            usage: wgpu::BufferUsages::VERTEX, // This buffer will be used as a vertex buffer
        });

        // Create the index buffer
        // This buffer contains indices that reference vertices in the vertex buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES), // Convert indices to bytes
            usage: wgpu::BufferUsages::INDEX, // This buffer will be used as an index buffer
        });

        let num_indices = INDICES.len() as u32;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // Now we specify the vertex buffer layout
                // This tells the pipeline what format our vertex data is in
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
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
            vertex_buffer,
            index_buffer,
            num_indices,
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
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // Bind the vertex buffer
            // Slot 0 corresponds to the first element in the vertex buffer layout array
            // The range 0.. means use all vertices in the buffer
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // Bind the index buffer
            // Specify the format (U16 or U32) and use all indices in the buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw using indexed rendering
            // draw_indexed(indices, base_vertex, instances)
            // - indices: range of indices to use from the index buffer
            // - base_vertex: value added to each index (useful for drawing multiple objects)
            // - instances: range of instances to draw (we're drawing 1 instance)
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

// Need to import BufferInitDescriptor trait
use wgpu::util::DeviceExt;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("wgpu Buffers and Indices")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(window));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        } => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                state.window().request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
