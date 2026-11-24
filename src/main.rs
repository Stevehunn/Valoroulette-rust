mod agent;
mod app;
mod persistence;
mod wheel;

use app::ValorouletteApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Valoroulette")
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([850.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Valoroulette",
        options,
        Box::new(|cc| Ok(Box::new(ValorouletteApp::new(cc)))),
    )
}
