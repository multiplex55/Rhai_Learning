mod examples;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rhai Learning",
        options,
        Box::new(|_cc| Ok(Box::new(ui::app::App::default()))),
    )
}
