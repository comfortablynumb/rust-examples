use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use serde::{Deserialize, Serialize};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Data Dashboard",
        options,
        Box::new(|cc| Ok(Box::new(Dashboard::new(cc)))),
    )
}

#[derive(Serialize, Deserialize)]
struct Dashboard {
    selected_tab: Tab,
    data_points: Vec<DataPoint>,
    new_value: f64,
    filter: String,
    show_grid: bool,
    plot_scale: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
enum Tab {
    Data,
    Visualization,
    Settings,
}

#[derive(Serialize, Deserialize, Clone)]
struct DataPoint {
    x: f64,
    y: f64,
    label: String,
}

impl Dashboard {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Generate some sample data
        let mut data_points = Vec::new();
        for i in 0..50 {
            let x = i as f64 * 0.1;
            data_points.push(DataPoint {
                x,
                y: (x * 2.0).sin() * 10.0 + (x * 0.5).cos() * 5.0,
                label: format!("Point {}", i),
            });
        }

        Self {
            selected_tab: Tab::Data,
            data_points,
            new_value: 0.0,
            filter: String::new(),
            show_grid: true,
            plot_scale: 1.0,
        }
    }

    fn data_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Management");
        ui.separator();

        // Filter
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.filter);
        });

        ui.separator();

        // Data table
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("data_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    // Header
                    ui.strong("Index");
                    ui.strong("X");
                    ui.strong("Y");
                    ui.strong("Label");
                    ui.strong("Actions");
                    ui.end_row();

                    // Data rows
                    let mut to_remove = None;
                    for (i, point) in self.data_points.iter_mut().enumerate() {
                        if !self.filter.is_empty()
                            && !point
                                .label
                                .to_lowercase()
                                .contains(&self.filter.to_lowercase())
                        {
                            continue;
                        }

                        ui.label(i.to_string());
                        ui.label(format!("{:.2}", point.x));
                        ui.label(format!("{:.2}", point.y));
                        ui.text_edit_singleline(&mut point.label);

                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }

                        ui.end_row();
                    }

                    if let Some(idx) = to_remove {
                        self.data_points.remove(idx);
                    }
                });
        });

        ui.separator();

        // Add new data point
        ui.horizontal(|ui| {
            ui.label("Add new value:");
            ui.add(egui::DragValue::new(&mut self.new_value).speed(0.1));
            if ui.button("Add").clicked() {
                let x = self.data_points.len() as f64 * 0.1;
                self.data_points.push(DataPoint {
                    x,
                    y: self.new_value,
                    label: format!("Point {}", self.data_points.len()),
                });
            }
        });

        ui.label(format!("Total points: {}", self.data_points.len()));
    }

    fn visualization_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Visualization");
        ui.separator();

        // Plot controls
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_grid, "Show Grid");
            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.plot_scale, 0.5..=2.0).text("x"));
        });

        ui.separator();

        // Plot
        let points: PlotPoints = self
            .data_points
            .iter()
            .map(|p| [p.x, p.y * self.plot_scale])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(100, 200, 100))
            .width(2.0);

        Plot::new("data_plot")
            .view_aspect(2.0)
            .show_grid(self.show_grid)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });

        ui.separator();

        // Statistics
        if !self.data_points.is_empty() {
            let y_values: Vec<f64> = self.data_points.iter().map(|p| p.y).collect();
            let mean = y_values.iter().sum::<f64>() / y_values.len() as f64;
            let min = y_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = y_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            ui.label(format!("Mean: {:.2}", mean));
            ui.label(format!("Min: {:.2}", min));
            ui.label(format!("Max: {:.2}", max));
        }
    }

    fn settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();

        ui.label("Application Settings");

        ui.separator();

        if ui.button("Clear All Data").clicked() {
            self.data_points.clear();
        }

        if ui.button("Generate Random Data").clicked() {
            self.data_points.clear();
            for i in 0..100 {
                let x = i as f64 * 0.05;
                self.data_points.push(DataPoint {
                    x,
                    y: (rand_value() - 0.5) * 20.0,
                    label: format!("Point {}", i),
                });
            }
        }

        ui.separator();

        ui.label("About");
        ui.label("A data dashboard built with egui");
        ui.hyperlink_to("egui GitHub", "https://github.com/emilk/egui");
    }
}

impl eframe::App for Dashboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with tabs
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::Data, "Data");
                ui.selectable_value(&mut self.selected_tab, Tab::Visualization, "Visualization");
                ui.selectable_value(&mut self.selected_tab, Tab::Settings, "Settings");
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| match self.selected_tab {
            Tab::Data => self.data_tab(ui),
            Tab::Visualization => self.visualization_tab(ui),
            Tab::Settings => self.settings_tab(ui),
        });
    }
}

// Simple random number generator
fn rand_value() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / 1_000_000_000.0).fract()
}
