# egui Application - Data Dashboard

A more complex egui application demonstrating tabs, data management, plotting, and state persistence.

## Features

- Multiple tabs/views
- Data table with filtering
- Interactive plots with egui_plot
- CRUD operations
- Settings management
- Responsive layout

## Code Examples

### Tab-Based Navigation

```rust
#[derive(PartialEq)]
enum Tab {
    Data,
    Visualization,
    Settings,
}

egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut self.selected_tab, Tab::Data, "Data");
        ui.selectable_value(&mut self.selected_tab, Tab::Visualization, "Visualization");
    });
});
```

### Data Table

```rust
egui::Grid::new("data_grid")
    .striped(true)
    .show(ui, |ui| {
        // Header
        ui.strong("Column 1");
        ui.strong("Column 2");
        ui.end_row();

        // Rows
        for item in &self.data {
            ui.label(&item.name);
            ui.label(item.value.to_string());
            ui.end_row();
        }
    });
```

### Plotting

```rust
use egui_plot::{Line, Plot, PlotPoints};

let points: PlotPoints = data.iter().map(|p| [p.x, p.y]).collect();
let line = Line::new(points);

Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(line);
});
```

## Running

```bash
cargo run
```

## References

- [egui_plot](https://docs.rs/egui_plot/)
- [egui Examples](https://github.com/emilk/egui/tree/master/examples)
