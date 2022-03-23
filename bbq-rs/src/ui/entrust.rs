use std::cmp::Ordering;
use std::collections::HashMap;
use eframe::egui::{Context, Grid, Ui, Window};
use bbq_core::{Entrust};

pub struct EntrustView {
    columns: Vec<&'static str>,
    order: HashMap<&'static str, Ordering>,
}

impl EntrustView {
    pub fn new() -> Self {
        let columns= vec![
            "代码", "名称", "委托时间",
            "类型",  "状态",
            "价格", "委托量","成交量", "撤销量",
            "备注",
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
    pub fn ui(&mut self, ctx: &Context,  entrusts: &mut Vec<Entrust>) {
        Window::new("委托单")
            .hscroll(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    Grid::new("trade.entrust")
                        .striped(true)
                        .num_columns(10)
                        .show(ui, |ui| {
                            for column in self.columns.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(*column);
                                });
                            }
                        });
                });
            });
    }
}
