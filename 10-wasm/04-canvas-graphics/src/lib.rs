use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Canvas wrapper for graphics operations
#[wasm_bindgen]
pub struct Canvas {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Canvas {
    /// Create a new Canvas instance from canvas element ID
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Canvas, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str("Canvas element not found"))?
            .dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width();
        let height = canvas.height();

        Ok(Canvas {
            context,
            width,
            height,
        })
    }

    /// Clear the entire canvas
    pub fn clear(&self) {
        self.context
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    /// Set fill color
    pub fn set_fill_color(&self, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
    }

    /// Set stroke color
    pub fn set_stroke_color(&self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(color));
    }

    /// Set line width
    pub fn set_line_width(&self, width: f64) {
        self.context.set_line_width(width);
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.context.fill_rect(x, y, width, height);
    }

    /// Draw a stroked rectangle
    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.context.stroke_rect(x, y, width, height);
    }

    /// Draw a filled circle
    pub fn fill_circle(&self, x: f64, y: f64, radius: f64) {
        self.context.begin_path();
        self.context
            .arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        self.context.fill();
    }

    /// Draw a stroked circle
    pub fn stroke_circle(&self, x: f64, y: f64, radius: f64) {
        self.context.begin_path();
        self.context
            .arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        self.context.stroke();
    }

    /// Draw a line
    pub fn draw_line(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.context.begin_path();
        self.context.move_to(x1, y1);
        self.context.line_to(x2, y2);
        self.context.stroke();
    }

    /// Draw text
    pub fn draw_text(&self, text: &str, x: f64, y: f64) {
        self.context.fill_text(text, x, y).unwrap();
    }

    /// Set font
    pub fn set_font(&self, font: &str) {
        self.context.set_font(font);
    }

    /// Begin a path
    pub fn begin_path(&self) {
        self.context.begin_path();
    }

    /// Move to a point
    pub fn move_to(&self, x: f64, y: f64) {
        self.context.move_to(x, y);
    }

    /// Draw line to a point
    pub fn line_to(&self, x: f64, y: f64) {
        self.context.line_to(x, y);
    }

    /// Close the current path
    pub fn close_path(&self) {
        self.context.close_path();
    }

    /// Fill the current path
    pub fn fill(&self) {
        self.context.fill();
    }

    /// Stroke the current path
    pub fn stroke(&self) {
        self.context.stroke();
    }

    /// Save context state
    pub fn save(&self) {
        self.context.save();
    }

    /// Restore context state
    pub fn restore(&self) {
        self.context.restore();
    }

    /// Translate the canvas
    pub fn translate(&self, x: f64, y: f64) {
        self.context.translate(x, y).unwrap();
    }

    /// Rotate the canvas
    pub fn rotate(&self, angle: f64) {
        self.context.rotate(angle).unwrap();
    }

    /// Scale the canvas
    pub fn scale(&self, x: f64, y: f64) {
        self.context.scale(x, y).unwrap();
    }
}

/// Draw a simple scene
#[wasm_bindgen]
pub fn draw_scene(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;

    // Clear canvas
    canvas.clear();

    // Draw background
    canvas.set_fill_color("#1a1a1a");
    canvas.fill_rect(0.0, 0.0, canvas.width as f64, canvas.height as f64);

    // Draw sun
    canvas.set_fill_color("#ffcc00");
    canvas.fill_circle(100.0, 100.0, 40.0);

    // Draw ground
    canvas.set_fill_color("#228b22");
    canvas.fill_rect(
        0.0,
        400.0,
        canvas.width as f64,
        canvas.height as f64 - 400.0,
    );

    // Draw house
    canvas.set_fill_color("#8b4513");
    canvas.fill_rect(250.0, 300.0, 150.0, 150.0);

    // Draw roof
    canvas.set_fill_color("#cd5c5c");
    canvas.begin_path();
    canvas.move_to(230.0, 300.0);
    canvas.line_to(325.0, 240.0);
    canvas.line_to(420.0, 300.0);
    canvas.close_path();
    canvas.fill();

    // Draw door
    canvas.set_fill_color("#654321");
    canvas.fill_rect(300.0, 370.0, 50.0, 80.0);

    // Draw window
    canvas.set_fill_color("#87ceeb");
    canvas.fill_rect(275.0, 330.0, 40.0, 40.0);
    canvas.fill_rect(345.0, 330.0, 40.0, 40.0);

    // Draw tree
    canvas.set_fill_color("#8b4513");
    canvas.fill_rect(500.0, 350.0, 30.0, 100.0);

    canvas.set_fill_color("#228b22");
    canvas.fill_circle(515.0, 330.0, 50.0);

    // Draw clouds
    canvas.set_fill_color("#ffffff");
    canvas.fill_circle(450.0, 80.0, 25.0);
    canvas.fill_circle(475.0, 75.0, 30.0);
    canvas.fill_circle(500.0, 80.0, 25.0);

    Ok(())
}

/// Animated bouncing ball
#[wasm_bindgen]
pub struct BouncingBall {
    canvas: Canvas,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
}

#[wasm_bindgen]
impl BouncingBall {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<BouncingBall, JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        Ok(BouncingBall {
            canvas,
            x: 100.0,
            y: 100.0,
            vx: 2.0,
            vy: 3.0,
            radius: 20.0,
        })
    }

    pub fn update(&mut self) {
        // Update position
        self.x += self.vx;
        self.y += self.vy;

        // Bounce off walls
        if self.x - self.radius < 0.0 || self.x + self.radius > self.canvas.width as f64 {
            self.vx = -self.vx;
        }
        if self.y - self.radius < 0.0 || self.y + self.radius > self.canvas.height as f64 {
            self.vy = -self.vy;
        }

        // Clamp position
        self.x = self
            .x
            .max(self.radius)
            .min(self.canvas.width as f64 - self.radius);
        self.y = self
            .y
            .max(self.radius)
            .min(self.canvas.height as f64 - self.radius);
    }

    pub fn draw(&self) {
        // Clear canvas
        self.canvas.clear();

        // Draw background
        self.canvas.set_fill_color("#f0f0f0");
        self.canvas.fill_rect(
            0.0,
            0.0,
            self.canvas.width as f64,
            self.canvas.height as f64,
        );

        // Draw ball
        self.canvas.set_fill_color("#ff6347");
        self.canvas.fill_circle(self.x, self.y, self.radius);

        // Draw shadow
        self.canvas.set_fill_color("rgba(0, 0, 0, 0.2)");
        self.canvas
            .fill_circle(self.x + 5.0, self.y + 5.0, self.radius);
    }
}
