// UI components module

// Public struct with public fields
pub struct Button {
    label: String,     // Private field
    pub enabled: bool, // Public field
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            label: label.to_string(),
            enabled: true,
        }
    }

    pub fn render(&self) {
        println!("  Rendering button: '{}'", self.label);
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    // Private method
    fn internal_update(&mut self) {
        // Some internal logic
    }
}

// Public struct
pub struct Window {
    title: String,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Window {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub fn show(&self) {
        println!(
            "  Showing window: '{}' ({}x{})",
            self.title, self.width, self.height
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

// Private struct - only accessible within this module
struct Layout {
    direction: LayoutDirection,
}

enum LayoutDirection {
    Horizontal,
    Vertical,
}

// Public enum
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn as_str(&self) -> &str {
        match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        }
    }
}

// Public function
pub fn render_all() {
    println!("  Rendering all UI components");
}

// Crate-visible function
pub(crate) fn internal_render() {
    println!("  Internal render function");
}

// Parent-visible function (visible to frontend module)
pub(super) fn parent_only() {
    println!("  Visible only to parent (frontend) module");
}

// Module-specific helper
fn calculate_layout() -> Layout {
    Layout {
        direction: LayoutDirection::Horizontal,
    }
}
