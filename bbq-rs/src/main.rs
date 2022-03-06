mod app;

// When compiling natively:
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let app = app::WrapApp::default();
    let options = eframe::NativeOptions {
        // Let's show off that we support transparent windows
        // transparent: true,
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
