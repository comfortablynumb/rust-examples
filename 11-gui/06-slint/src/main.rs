slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Handle button clicks
    let ui_weak = ui.as_weak();
    ui.on_request_increase(move || {
        let ui = ui_weak.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    let ui_weak = ui.as_weak();
    ui.on_request_decrease(move || {
        let ui = ui_weak.unwrap();
        ui.set_counter(ui.get_counter() - 1);
    });

    let ui_weak = ui.as_weak();
    ui.on_request_reset(move || {
        let ui = ui_weak.unwrap();
        ui.set_counter(0);
    });

    ui.run()
}
