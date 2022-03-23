use std::sync::{Arc, RwLock};
use std::time::Duration;
use ui::app;

mod font;
mod ui;
use ui::app::QApp;

mod store;
mod event;

use tokio::sync::mpsc;
use tokio::sync::broadcast;
use futures::future::FutureExt;
use crate::event::TraderEvent;
use crate::store::Store;


fn main() {
    tracing_subscriber::fmt::init();

    let (event_tx, mut event_rx ) = mpsc::unbounded_channel();
    let (broadcast_tx, _) = broadcast::channel(16);

    let tk_event_tx = event_tx.clone();
    let mut tk_broadcast_rx = broadcast_tx.subscribe();
    let tk_handler = std::thread::spawn(move || {
        // tokio thread
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.enable_all().build().expect("tokio rt").block_on(async move  {
            loop {
                let to = tokio::time::sleep(Duration::from_secs(5)).boxed();

                tokio::select! {
                    val = tk_broadcast_rx.recv() => {
                        if let Ok(v) = val {
                            println!("receive broadcast, break: {:?}", v);
                        }
                    },
                    _ = to => {
                        // tk_event_tx.send(CoreEvent::Test("Event from Tokio".to_string()));
                        println!("recv timout")
                    }
                }
            }
        })
    });

    let app = QApp::new(broadcast_tx, event_rx);

    let options = eframe::NativeOptions {
        // transparent: true,
        drag_and_drop_support: true,
        initial_window_size: Some(eframe::egui::Vec2::new(1024.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
