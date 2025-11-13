# egui Basics

Introduction to egui, an easy-to-use immediate mode GUI library for Rust.

## Concepts Covered

- Immediate mode GUI concept
- Basic widgets (buttons, sliders, checkboxes)
- Text input and editing
- Layouts (horizontal, vertical)
- Color pickers
- Collapsing headers
- Event handling

## What is Immediate Mode?

Unlike retained mode GUIs (like HTML/CSS), immediate mode GUIs:
- Rebuild the UI every frame
- No separate UI state to manage
- Simple and intuitive API
- Great for tools and debug UIs

## Code Examples

### Basic Structure

```rust
use eframe::egui;

struct MyApp {
    name: String,
    age: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI code here
        });
    }
}
```

### Widgets

```rust
// Text input
ui.text_edit_singleline(&mut self.name);

// Number input
ui.add(egui::DragValue::new(&mut self.age));

// Slider
ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0));

// Checkbox
ui.checkbox(&mut self.checkbox, "Check me!");

// Button
if ui.button("Click me!").clicked() {
    println!("Clicked!");
}

// Radio buttons
ui.radio_value(&mut self.choice, Choice::A, "Option A");
ui.radio_value(&mut self.choice, Choice::B, "Option B");
```

### Layouts

```rust
// Horizontal layout
ui.horizontal(|ui| {
    ui.label("Label:");
    ui.button("Button");
});

// Vertical layout (default)
ui.vertical(|ui| {
    ui.label("First");
    ui.label("Second");
});

// Grid layout
egui::Grid::new("my_grid").show(ui, |ui| {
    ui.label("Row 1, Col 1");
    ui.label("Row 1, Col 2");
    ui.end_row();
    ui.label("Row 2, Col 1");
    ui.label("Row 2, Col 2");
    ui.end_row();
});
```

### Styling

```rust
// Color picker
ui.color_edit_button_srgba(&mut self.color);

// Custom spacing
ui.spacing_mut().item_spacing.x = 10.0;

// Separator
ui.separator();
```

## Running

```bash
cargo run
```

## Features

- **Cross-platform**: Windows, macOS, Linux, Web (WASM)
- **No dependencies**: Except for windowing (eframe)
- **Immediate mode**: Simple mental model
- **Rich widgets**: Buttons, sliders, plots, etc.
- **Customizable**: Themes and styling

## When to Use egui

**Good for:**
- Developer tools and editors
- Debug UIs
- Game development tools
- Data visualization apps
- Prototypes

**Not ideal for:**
- Complex web-like layouts
- Document-based interfaces
- Apps requiring pixel-perfect design

## Key Concepts

### The `ui` Object

The `ui` parameter is your interface to egui:
- Add widgets with `ui.button()`, `ui.label()`, etc.
- Get responses: `if ui.button("Click").clicked() { ... }`
- Layout control: `ui.horizontal()`, `ui.vertical()`

### Responses

Widgets return `Response` objects:
```rust
let response = ui.button("Click me");
if response.clicked() {
    // Button was clicked
}
if response.hovered() {
    // Mouse is hovering
}
```

### Context

The `Context` (`ctx`) is the egui state:
```rust
ctx.request_repaint();  // Request another frame
ctx.set_pixels_per_point(2.0);  // Set DPI scale
```

## References

- [egui Documentation](https://docs.rs/egui/)
- [egui Demo](https://www.egui.rs/)
- [egui GitHub](https://github.com/emilk/egui)
