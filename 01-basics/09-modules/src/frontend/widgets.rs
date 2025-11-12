// Widget components module

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
        }
    }

    pub fn display(&self) {
        println!("  Label: '{}'", self.text);
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub struct TextBox {
    content: String,
    max_length: usize,
}

impl TextBox {
    pub fn new(max_length: usize) -> Self {
        TextBox {
            content: String::new(),
            max_length,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        if content.len() <= self.max_length {
            self.content = content.to_string();
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn render(&self) {
        println!("  TextBox: '{}'", self.content);
    }
}

pub struct Slider {
    value: f64,
    min: f64,
    max: f64,
}

impl Slider {
    pub fn new(min: f64, max: f64) -> Self {
        Slider {
            value: min,
            min,
            max,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        if value >= self.min && value <= self.max {
            self.value = value;
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

// Private helper struct
struct WidgetState {
    focused: bool,
    hovered: bool,
}

impl WidgetState {
    fn new() -> Self {
        WidgetState {
            focused: false,
            hovered: false,
        }
    }
}

// Public function
pub fn render_widgets() {
    println!("  Rendering all widgets");
}

// Private function
fn calculate_bounds() {
    // Internal calculation
}
