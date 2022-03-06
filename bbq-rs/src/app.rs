use eframe::egui::{self, Context, Rgba, Visuals};
use eframe::epi;
use eframe::epi::{Frame, Storage};

#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WrapApp {
}

impl epi::App for WrapApp {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.heading("😃😃Demo application😃😃");
            });
        });
        egui::TopBottomPanel::bottom("wrap_app_bottom").show(ctx, |ui| {
            ui.label("this is a status bottom");
        });
    }

    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
        ctx.set_visuals(Visuals::dark())
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "bbq-rs"
    }

    fn clear_color(&self) -> Rgba {
        Rgba::TRANSPARENT
    }
}
