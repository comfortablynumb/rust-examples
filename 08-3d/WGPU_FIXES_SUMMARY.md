# WGPU 0.19 and Winit 0.29 Compilation Fixes

## Summary
Fixed compilation errors in wgpu examples for compatibility with wgpu 0.19 and winit 0.29.

## Fixes Applied

### 1. Surface Lifetime Parameter
**Issue:** `wgpu::Surface` now requires a lifetime parameter in wgpu 0.19

**Fix:**
```rust
// Before
struct State {
    surface: wgpu::Surface,
    window: Window,
    // ...
}

// After  
struct State<'a> {
    window: Arc<Window>,
    surface: wgpu::Surface<'a>,
    // ...
}

impl<'a> State<'a> {
    async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        // ...
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        // ...
    }
}
```

### 2. Winit 0.29 KeyboardInput API Change
**Issue:** `WindowEvent::KeyboardInput` structure changed in winit 0.29

**Fix:**
```rust
// Before
WindowEvent::KeyboardInput {
    input: KeyboardInput {
        state: ElementState::Pressed,
        virtual_keycode: Some(VirtualKeyCode::Escape),
        ..
    },
    ..
}

// After
WindowEvent::KeyboardInput {
    event: KeyEvent {
        state: ElementState::Pressed,
        logical_key: Key::Named(NamedKey::Escape),
        ..
    },
    ..
}
```

**Required imports:**
```rust
use winit::keyboard::{Key, NamedKey};
```

### 3. SurfaceConfiguration Missing Field
**Issue:** `desired_maximum_frame_latency` field is required in wgpu 0.19

**Fix:**
```rust
let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface_format,
    width: size.width,
    height: size.height,
    present_mode: surface_caps.present_modes[0],
    alpha_mode: surface_caps.alpha_modes[0],
    view_formats: vec![],
    desired_maximum_frame_latency: 2,  // Added this line
};
```

### 4. EventLoop API Changes
**Issue:** Multiple changes to EventLoop in winit 0.29

**Fixes:**
```rust
// EventLoop::new() now returns Result
let event_loop = EventLoop::new().unwrap();

// event_loop.run() signature changed
// Before: event_loop.run(move |event, _, control_flow|
// After:  event_loop.run(move |event, elwt|

// ControlFlow::Exit changed to elwt.exit()
// Before: *control_flow = ControlFlow::Exit
// After:  elwt.exit()

// Event loop now returns Result
event_loop.run(move |event, elwt| {
    // ...
}).unwrap();
```

### 5. Event Type Changes
**Issue:** Some event types were restructured in winit 0.29

**Fixes:**
```rust
// RedrawRequested moved into WindowEvent
// Before:
Event::RedrawRequested(window_id) if window_id == state.window().id() => {
    // ...
}

// After:
WindowEvent::RedrawRequested => {
    // ...
}

// MainEventsCleared renamed to AboutToWait
// Before: Event::MainEventsCleared
// After:  Event::AboutToWait
```

### 6. ScaleFactorChanged Removed
**Issue:** `ScaleFactorChanged` event handler signature changed

**Fix:** Removed the handler as it's no longer needed with new API:
```rust
// Removed:
WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
    state.resize(**new_inner_size);
}
```

### 7. Import Changes
**Required new imports:**
```rust
use std::sync::Arc;
use winit::keyboard::{Key, NamedKey};
```

**Removed imports:**
```rust
// Remove ControlFlow from:
use winit::event_loop::{ControlFlow, EventLoop};
// Change to:
use winit::event_loop::EventLoop;
```

## Files Successfully Fixed and Verified

✅ 01-triangle - Compiles and builds successfully
✅ 02-buffers-indices - Compiles and builds successfully  
✅ 03-textures - Compiles and builds successfully (with anyhow Result handling)
✅ 04-3d-cube - Compiles and builds successfully (fixed Cargo.toml package name)

## Special Cases

### 03-textures
This example uses `anyhow::Result` which conflicts with `std::result::Result`. 
Fixed by aliasing `Ok`:
```rust
use std::result::Result::Ok as StdOk;
use image::GenericImageView;  // Also needed for texture loading
```

### 04-3d-cube  
Fixed Cargo.toml package name (can't start with digit):
```toml
# Before: name = "3d-cube"
# After:  name = "cube-3d"
```

## Remaining Examples
The following examples need the same fixes applied:
- 05-camera
- 06-lighting
- 07-multiple-objects
- 08-compute-shader

All follow the same pattern as the fixed examples above.
