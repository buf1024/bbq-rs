use eframe::egui::{CollapsingHeader, Context, Grid, Layout, ScrollArea, SidePanel, Ui, Vec2, Window};
use bbq_core::trader::{Account};

pub struct AccountView {}

impl AccountView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ui(&mut self, ctx: &Context, account: &mut Account) {
        SidePanel::right("trade.account")
            // .min_width(200.0)

            .show(ctx, |ui| {
                ui.heading(format!("账户: {}", account.account_id.as_str()));
                ui.separator();
                ScrollArea::new([false, true]).show(ui, |ui| {
                    Grid::new("trade.account.summary")
                        .striped(true)
                        .num_columns(4)
                        .spacing(Vec2::new(10.0, 10.0))
                        .show(ui, |ui| {
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("账户状态:"));
                            ui.label(account.status.to_string());
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("交易类别:"));
                            ui.label(account.kind.to_string());

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("运行模式:"));
                            ui.label(account.kind.to_string());
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("券商手续费:"));
                            ui.label(format!("{:.4}", account.broker_fee));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("过户费:"));
                            ui.label(format!("{:.4}", account.transfer_fee));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("印花税:"));
                            ui.label(format!("{:.4}", account.tax_fee));
                            ui.end_row();
                        });
                    ui.separator();
                    Grid::new("trade.account.money")
                        .striped(true)
                        .num_columns(4)
                        .spacing(Vec2::new(10.0, 10.0))
                        .show(ui, |ui| {
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("初始资金:"));
                            ui.label(format!("{:.4}", account.cash_init));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("可用资金:"));
                            ui.label(format!("{:.4}", account.cash_available));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("冻结资金:"));
                            ui.label(format!("{:.4}", account.cash_frozen));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("净值:"));
                            ui.label(format!("{:.4}", account.total_net_value));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui| ui.label("市值:"));
                            ui.label(format!("{:.4}", account.total_hold_value));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui| ui.label("持仓成本:"));
                            ui.label(format!("{:.4}", account.cost));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("持仓盈亏:"));
                            ui.label(format!("{:.4}", account.profit));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("总盈亏:"));
                            ui.label(format!("{:.4}", account.profit_rate));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("平仓盈亏:"));
                            ui.label(format!("{:.4}", account.close_profit));
                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("持仓盈比例:"));
                            ui.label(format!("{:.4}", account.total_profit));
                            ui.end_row();

                            ui.with_layout(Layout::right_to_left(),
                                           |ui|
                                               ui.label("总盈亏比例:"));
                            ui.label(format!("{:.4}", account.total_profit_rate));
                            ui.end_row();
                        });
                    ui.separator();
                    CollapsingHeader::new("策略配置")
                        .show(ui, |ui| {
                            Grid::new("trade.account.strategy")
                                .striped(true)
                                .num_columns(4)
                                .spacing(Vec2::new(10.0, 10.0))
                                .show(ui, |ui| {
                                    ui.with_layout(Layout::right_to_left(),
                                                   |ui|
                                                       ui.label("交易策略:"));
                                    // ui.label(account.strategy_name.to_string());

                                    if ui.button("参数").clicked() {}
                                    ui.end_row();

                                    ui.with_layout(Layout::right_to_left(),
                                                   |ui|
                                                       ui.label("风控策略:"));
                                    // ui.label(account.risk_name.to_string());
                                    if ui.button("参数").clicked() {}
                                    ui.end_row();

                                    ui.with_layout(Layout::right_to_left(),
                                                   |ui|
                                                       ui.label("交易接口:"));
                                    // ui.label(account.broker_name.to_string());
                                    if ui.button("参数").clicked() {}
                                    ui.end_row();
                                });
                        });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            if ui.button("结束").clicked() {}
                            if ui.button("暂停").clicked() {}
                        });
                    });
                });
            });
    }
}
