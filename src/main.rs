use eframe::egui;
use skyradar::app::SkyRadarApp;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Set up the native options for the window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(true),
        ..Default::default()
    };

    // Launch the application
    eframe::run_native(
        "SkyRadar - Live Aircraft Tracker",
        options,
        Box::new(|_cc| Box::new(SkyRadarApp::new())),
    )
} 