use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui Basics",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    name: String,
    age: u32,
    slider_value: f32,
    checkbox: bool,
    radio: Enum,
    text: String,
    color: egui::Color32,
}

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            slider_value: 50.0,
            checkbox: false,
            radio: Enum::First,
            text: String::new(),
            color: egui::Color32::from_rgb(100, 200, 100),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to egui!");

            ui.separator();

            // Text input
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            // Number input
            ui.horizontal(|ui| {
                ui.label("Your age: ");
                ui.add(egui::DragValue::new(&mut self.age).speed(0.1));
            });

            ui.separator();

            // Slider
            ui.label("Slider:");
            ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=100.0).text("value"));

            ui.separator();

            // Checkbox
            ui.checkbox(&mut self.checkbox, "Check me!");

            ui.separator();

            // Radio buttons
            ui.label("Radio buttons:");
            ui.radio_value(&mut self.radio, Enum::First, "First");
            ui.radio_value(&mut self.radio, Enum::Second, "Second");
            ui.radio_value(&mut self.radio, Enum::Third, "Third");

            ui.separator();

            // Multi-line text
            ui.label("Multi-line text:");
            ui.text_edit_multiline(&mut self.text);

            ui.separator();

            // Color picker
            ui.label("Color picker:");
            ui.color_edit_button_srgba(&mut self.color);

            // Show a colored rectangle
            let rect = ui.available_rect_before_wrap();
            let rect = egui::Rect::from_min_size(rect.min, egui::vec2(100.0, 50.0));
            ui.painter().rect_filled(rect, 0.0, self.color);
            ui.add_space(60.0);

            ui.separator();

            // Button
            if ui.button("Click me!").clicked() {
                println!("Button clicked!");
                println!("Name: {}, Age: {}", self.name, self.age);
            }

            ui.separator();

            // Display values
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.label(format!("Slider value: {:.1}", self.slider_value));
            ui.label(format!("Checkbox: {}", self.checkbox));
            ui.label(format!("Radio: {:?}", self.radio));

            ui.separator();

            // Collapsing header
            ui.collapsing("Click to expand", |ui| {
                ui.label("This is hidden content");
                ui.label("You can put anything here");
            });

            ui.separator();

            // Horizontal layout
            ui.horizontal(|ui| {
                ui.label("Buttons:");
                if ui.button("One").clicked() {
                    println!("One");
                }
                if ui.button("Two").clicked() {
                    println!("Two");
                }
                if ui.button("Three").clicked() {
                    println!("Three");
                }
            });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                });
            });
        });
    }
}
