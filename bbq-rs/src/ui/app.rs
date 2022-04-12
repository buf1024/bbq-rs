use std::borrow::BorrowMut;
use std::collections::{BTreeSet, HashMap};
use std::fmt::format;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use eframe::egui::{self, Window, Color32, Context, Id, Layout, Rgba, TextFormat, Visuals, CollapsingHeader, Ui, RichText, Vec2};
use eframe::egui::text::LayoutJob;
use eframe::epi;
use eframe::epi::{Frame, Storage};
use crate::store::{Module, Settings, Store};
use tokio::sync::{mpsc, broadcast};
use crate::event::{TraderEvent, CtrlEvent};
use crate::ui::data::DataView;
use crate::ui::setting::SettingView;
use crate::ui::trade::TradeView;
use crate::ui::View;

pub struct QApp {

    store: Store,
    store_t: Arc<RwLock<Store>>,

    broadcast_tx: broadcast::Sender<CtrlEvent>,
    event_rx: Option<mpsc::UnboundedReceiver<TraderEvent>>,

    views: HashMap<Module, Box<dyn View>>,
}


impl QApp {
    pub fn new(broadcast_tx: broadcast::Sender<CtrlEvent>, mut event_rx: mpsc::UnboundedReceiver<TraderEvent>) -> Self {

        let mut views: HashMap<Module, Box<dyn View>> = HashMap::new();
        views.insert(Module::Setting, Box::new(SettingView::new()));
        views.insert(Module::Data, Box::new(DataView::new()));
        views.insert(Module::Backtest, Box::new(TradeView::new(Module::Backtest)));
        views.insert(Module::Trade, Box::new(TradeView::new(Module::Trade)));


        let store = Store::default();
        Self {
            broadcast_tx,
            event_rx: Some(event_rx),
            store_t: Arc::new(RwLock::new(store.clone())),

            store,
            views,
        }
    }

    fn event_task(&self, frame: Frame,
                  store: Arc<RwLock<Store>>,
                  mut event_rx: mpsc::UnboundedReceiver<TraderEvent>) {
        std::thread::spawn(move || {
            let mut loop_count = 0;
            while let Some(event) = event_rx.blocking_recv() {
                match event {
                    TraderEvent::Test(val) => {
                        let mut store = store.write().unwrap();
                        println!("ui receive: {}, loop_count: {}", val, loop_count);
                        store.settings.db_url = format!("new store event: {}", loop_count);
                    }
                }
                loop_count = loop_count + 1;
                frame.request_repaint()
            }
        });
    }
    // fn setup_view(&'a mut self) {
    //     self.views.insert(Module::Setting, Box::new(SettingView::new(&mut store.settings)));
    //
    // }
    pub fn side_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("left")
            .min_width(35.0)
            .max_width(35.0)
            .default_width(35.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.selectable_label(self.store.module == Module::Data, "数据")
                        .clicked() {
                        self.store.module = Module::Data;
                    }
                    if ui.selectable_label(self.store.module == Module::Analyse, "投研")
                        .clicked() {
                        self.store.module = Module::Analyse;
                    }
                    if ui.selectable_label(self.store.module == Module::Backtest, "回测")
                        .clicked() {
                        self.store.module = Module::Backtest;
                    }
                    if ui.selectable_label(self.store.module == Module::Trade, "交易")
                        .clicked() {
                        self.store.module = Module::Trade;
                    }
                    ui.separator();
                    if ui.button("设置")
                        .clicked() {
                        self.store.settings.show = !self.store.settings.show;
                    };
                    // if ui.button("退出")
                    //     .clicked() {
                    //     frame.quit();
                    // }
                });
            });
    }
}

impl epi::App for QApp {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        {
            let store_t = self.store_t.read().unwrap();
            self.store = store_t.deref().clone();
        }
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.heading("实例程序")
            });
        });

        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut job = LayoutJob::default();
                job.append(
                    "上证指数 ",
                    0.0,
                    TextFormat {
                        color: Color32::LIGHT_GRAY,
                        ..Default::default()
                    },
                );
                job.append(
                    "3350.0",
                    0.0,
                    TextFormat {
                        color: Color32::RED,
                        ..Default::default()
                    },
                );
                ui.label(job);

                let mut job = LayoutJob::default();
                job.append(
                    "上证指数 ",
                    0.0,
                    TextFormat {
                        color: Color32::LIGHT_GRAY,
                        ..Default::default()
                    },
                );
                job.append(
                    "3350.0",
                    0.0,
                    TextFormat {
                        color: Color32::RED,
                        ..Default::default()
                    },
                );
                ui.label(job);

            });
        });

        self.side_panel(ctx);

        if self.views.contains_key(&self.store.module) {
            self.views.get_mut(&self.store.module).unwrap().show(ctx, &mut self.store);
        }

        if self.store.settings.show {
            // SettingView::new().show(ctx, &mut self.store.settings);
            self.views.get_mut(&Module::Setting).unwrap().show(ctx, &mut self.store);

        }
        {
            let mut store = self.store_t.write().unwrap();
            *store = self.store.clone();
        }
    }

    fn setup(&mut self, ctx: &Context, frame: &Frame, _storage: Option<&dyn Storage>) {
        if let Some(storage) = _storage {
            self.store = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
        crate::font::install_fonts(ctx);
        ctx.set_visuals(Visuals::dark());


        let mut event_rx = self.event_rx.take().unwrap();
        self.event_task(frame.clone(),
                        self.store_t.clone(),
                        event_rx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.store);
    }

    fn name(&self) -> &str {
        "bbq-rs"
    }

    fn clear_color(&self) -> Rgba {
        Rgba::TRANSPARENT
    }
    fn on_exit(&mut self) {}
}


/*
/// fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
///     let image = image::load_from_memory(image_data)?;
///     let size = [image.width() as _, image.height() as _];
///     let image_buffer = image.to_rgba8();
///     let pixels = image_buffer.as_flat_samples();
///     Ok(ColorImage::from_rgba_unmultiplied(
///         size,
///         pixels.as_slice(),
///     ))
/// }
*/
