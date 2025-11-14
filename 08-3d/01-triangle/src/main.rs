use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};

/// Represents the GPU state and rendering pipeline
/// This struct encapsulates all the wgpu objects needed for rendering
struct State<'a> {
    // The window must outlive the surface
    window: Arc<Window>,

    // Surface represents the platform-specific window surface (e.g., X11, Wayland, Windows)
    surface: wgpu::Surface<'a>,

    // Device is the logical GPU interface - used to create GPU resources
    device: wgpu::Device,

    // Queue is used to submit commands to the GPU
    queue: wgpu::Queue,

    // Configuration for the surface (format, size, present mode)
    config: wgpu::SurfaceConfiguration,

    // Window size
    size: winit::dpi::PhysicalSize<u32>,

    // Render pipeline defines how vertices are processed and pixels are rendered
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    /// Initialize the GPU state and create the render pipeline
    /// This is where we set up the entire graphics pipeline
    async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        // Instance is the entry point to wgpu - it represents a handle to the GPU
        // Backends::all() means we'll try Vulkan, Metal, DX12, or WebGPU depending on platform
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // SAFETY: The surface needs to live as long as the window that created it.
        // State owns the window, so this is safe.
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        // Adapter represents a physical GPU. We request one that's compatible with our surface
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Request a logical device and command queue from the adapter
        // Features: optional GPU capabilities we want to enable
        // Limits: constraints on resource sizes (buffer size, texture dimensions, etc.)
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

        // Get the surface capabilities (supported formats, present modes, alpha modes)
        let surface_caps = surface.get_capabilities(&adapter);

        // Choose a surface format. Prefer sRGB for correct color display
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Configure the surface with the chosen format and size
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

        // Load and compile the shader
        // WGSL (WebGPU Shading Language) is the shader language for wgpu
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create the render pipeline layout
        // Layout defines the structure of bind groups (uniforms, textures, etc.)
        // For this simple example, we have no bind groups
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // Create the render pipeline
        // This is the core of the graphics pipeline - it defines:
        // 1. How vertices are processed (vertex shader)
        // 2. How fragments/pixels are colored (fragment shader)
        // 3. How primitives are assembled (triangles, lines, etc.)
        // 4. Depth testing, blending, multisampling, etc.
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            // Vertex stage: processes each vertex
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // Function name in shader
                buffers: &[],           // We're using hardcoded vertices in the shader
            },

            // Fragment stage: determines the color of each pixel
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // No blending, just replace
                    write_mask: wgpu::ColorWrites::ALL,     // Write all color channels (RGBA)
                })],
            }),

            // How to interpret the vertex data as primitives
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Every 3 vertices = 1 triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // Counter-clockwise vertices are front-facing
                cull_mode: Some(wgpu::Face::Back), // Don't render back-facing triangles
                polygon_mode: wgpu::PolygonMode::Fill, // Fill the triangle (vs. lines or points)
                unclipped_depth: false,
                conservative: false,
            },

            // No depth/stencil buffer for this simple example
            depth_stencil: None,

            // No multisampling
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            // No multiview rendering
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
        }
    }

    /// Get a reference to the window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Handle window resize events
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Handle input events (currently unused, but here for future expansion)
    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    /// Update game state (currently unused, but here for future expansion)
    fn update(&mut self) {
        // Nothing to update for a static triangle
    }

    /// Render a frame
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Get the next frame from the surface
        // This is a texture we can render to
        let output = self.surface.get_current_texture()?;

        // Create a view into the texture
        // Views describe how to access the texture
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder to record GPU commands
        // Think of this as recording a list of instructions to send to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Begin a render pass
        // A render pass is a sequence of draw commands that render to a set of textures
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

            // Set the render pipeline
            render_pass.set_pipeline(&self.render_pipeline);

            // Draw 3 vertices (1 triangle)
            // Vertex data is hardcoded in the shader
            render_pass.draw(0..3, 0..1);
        }

        // Submit the command buffer to the GPU queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present the rendered frame to the screen
        output.present();

        Ok(())
    }
}

fn main() {
    // Initialize logging
    env_logger::init();

    // Create the event loop and window
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("wgpu Triangle")
        .build(&event_loop)
        .unwrap();

    // Initialize GPU state (async operation)
    let mut state = pollster::block_on(State::new(window));

    // Main event loop
    event_loop
        .run(move |event, elwt| match event {
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
        })
        .unwrap();
}
