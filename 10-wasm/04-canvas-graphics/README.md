# Canvas Graphics with WebAssembly

Demonstrates drawing graphics on HTML Canvas using Rust and WebAssembly.

## Concepts Covered

- Canvas 2D rendering context
- Drawing shapes (rectangles, circles, lines)
- Path-based drawing
- Transformations (translate, rotate, scale)
- Animation with WASM
- Canvas state management

## Code Examples

### Canvas Wrapper

```rust
#[wasm_bindgen]
pub struct Canvas {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Canvas {
    pub fn fill_circle(&self, x: f64, y: f64, radius: f64) {
        self.context.begin_path();
        self.context.arc(x, y, radius, 0.0, 2.0 * PI).unwrap();
        self.context.fill();
    }
}
```

### Drawing Shapes

```rust
// Rectangle
canvas.set_fill_color("#ff0000");
canvas.fill_rect(50.0, 50.0, 100.0, 80.0);

// Circle
canvas.set_fill_color("#00ff00");
canvas.fill_circle(200.0, 200.0, 40.0);

// Custom path
canvas.begin_path();
canvas.move_to(250.0, 300.0);
canvas.line_to(325.0, 240.0);
canvas.line_to(420.0, 300.0);
canvas.close_path();
canvas.fill();
```

### Animation

```rust
#[wasm_bindgen]
pub struct BouncingBall {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[wasm_bindgen]
impl BouncingBall {
    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        // Bounce logic
    }

    pub fn draw(&self) {
        canvas.fill_circle(self.x, self.y, 20.0);
    }
}
```

## Building

```bash
wasm-pack build --target web
```

## HTML Setup

```html
<!DOCTYPE html>
<html>
<head>
    <title>Canvas Graphics</title>
</head>
<body>
    <canvas id="myCanvas" width="600" height="400"></canvas>

    <script type="module">
        import init, { draw_scene, BouncingBall } from './pkg/canvas_graphics.js';

        async function run() {
            await init();

            // Draw static scene
            draw_scene('myCanvas');

            // Or create animation
            const ball = new BouncingBall('myCanvas');

            function animate() {
                ball.update();
                ball.draw();
                requestAnimationFrame(animate);
            }

            animate();
        }

        run();
    </script>
</body>
</html>
```

## Performance Tips

1. **Minimize Context Operations**: Cache rendering context
2. **Batch Drawing**: Group similar operations together
3. **Clear Efficiently**: Clear only what needs updating
4. **Use requestAnimationFrame**: For smooth animations
5. **Optimize Paths**: Reduce number of path operations

## Transformations

```rust
// Save state
canvas.save();

// Translate and rotate
canvas.translate(100.0, 100.0);
canvas.rotate(45.0 * PI / 180.0);

// Draw
canvas.fill_rect(-25.0, -25.0, 50.0, 50.0);

// Restore state
canvas.restore();
```

## References

- [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [web-sys Canvas Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
