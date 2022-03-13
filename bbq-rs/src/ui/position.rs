use std::cmp::Ordering;
use std::collections::HashMap;
use eframe::egui::{Context, Grid, Ui, Window};
use bbq_core::Position;
use crate::Store;
use crate::ui::View;

pub struct PositionView {
    columns: Vec<&'static str>,
    order: HashMap<&'static str, Ordering>,
}

impl PositionView {
    pub fn new() -> Self {
        let columns= vec![
            "代码", "名称", "建仓时间",
            "持仓量",  "可用持仓量", "冻结持仓量",
            "持仓手续费", "平均持仓价",
            "最新价", "最高价", "最低价",
            "盈利(%)", "最大盈利(%@)", "最小盈利(%@)",
        ];
        let mut order = HashMap::new();
        for column in columns.iter() {
            order.insert(*column, Ordering::Equal);
        }
        Self{
            columns,
            order
        }
    }
    pub fn ui(&mut self, ctx: &Context, positions: &mut HashMap<String, Position>) {
        Window::new("持仓头寸")
            .hscroll(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    Grid::new("trade.position")
                        .striped(true)
                        .num_columns(14)
                        .show(ui, |ui| {
                            for column in self.columns.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(*column);
                                    // if ui.button("🔻🔺").clicked() {
                                    //
                                    // }
                                });
                            }
                        });
                });
            });
    }
}
