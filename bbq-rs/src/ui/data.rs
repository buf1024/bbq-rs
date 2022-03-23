use eframe::egui::{Button, Context, Grid, Layout, ScrollArea, SidePanel};
use crate::Store;
use crate::ui::View;

pub struct DataView {}

impl DataView {
    pub fn new() -> Self {
        Self{}
    }
}

impl View for DataView {
    fn show(&mut self, ctx: &Context, store: &mut Store) {
        SidePanel::left("data.setting")
            .show(ctx, |ui| {
                ScrollArea::new([true, true]).show(ui, |ui| {
                    ui.heading("股票数据设置");
                    ui.separator();
                    Grid::new("data.stock.setting")
                        .num_columns(5)
                        .striped(true)
                        .show(ui, |ui| {
                            for coll in store.data.stock_coll.iter() {
                                ui.label("文档:");
                                ui.label(coll.coll_name.as_str());
                                ui.label("同步时间:");
                                ui.label(coll.last_sync.as_str());
                                if !coll.is_latest {
                                    if ui.button("同步").clicked() {

                                    }
                                } else {
                                    ui.add_enabled(false, Button::new("同步"));
                                }
                            }
                            ui.end_row();
                        });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("自动同步时间(HH:MM:SS):");
                        ui.text_edit_singleline(&mut store.data.stock_auto_sync);
                    });
                    ui.separator();
                    ui.horizontal(|ui|{
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            if ui.button("日志").clicked() {

                            }
                            if ui.button("全量同步").clicked() {

                            }
                        });
                    });

                    ui.separator();

                    ui.heading("基金数据设置");
                    ui.separator();
                    Grid::new("data.fund.setting")
                        .num_columns(5)
                        .striped(true)
                        .show(ui, |ui| {
                            for coll in store.data.fund_coll.iter() {
                                ui.label("文档:");
                                ui.label(coll.coll_name.as_str());
                                ui.label("同步时间:");
                                ui.label(coll.last_sync.as_str());
                                if !coll.is_latest {
                                    if ui.button("同步").clicked() {

                                    }
                                } else {
                                    ui.add_enabled(false, Button::new("同步"));
                                }
                            }
                            ui.end_row();
                        });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("自动同步时间(HH:MM:SS):");
                        ui.text_edit_singleline(&mut store.data.fund_auto_sync);
                    });
                    ui.separator();
                    ui.horizontal(|ui|{
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            if ui.button("日志").clicked() {

                            }
                            if ui.button("全量同步").clicked() {

                            }
                        });
                    });
                });
            });
    }
}
