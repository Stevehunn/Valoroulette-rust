mod agent;
mod app;
mod i18n;
mod persistence;
mod style;
mod update;
mod wheel;

use app::ValorouletteApp;
use eframe::egui;

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Icon not found in assets/icon.png")
        .to_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> eframe::Result<()> {
    let icon = load_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Valoroulette")
            .with_inner_size([1180.0, 800.0])
            .with_min_inner_size([900.0, 640.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Valoroulette",
        options,
        Box::new(|cc| Ok(Box::new(ValorouletteApp::new(cc)))),
    )
}