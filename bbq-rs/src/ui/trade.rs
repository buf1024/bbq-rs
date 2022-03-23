use eframe::egui::{self, Vec2, CollapsingHeader, ComboBox, Context, Grid, Layout, TextEdit, TopBottomPanel, Ui, Window};
use crate::Store;
use crate::store::{Module, Settings, TreePath};
use crate::ui::account::AccountView;
use crate::ui::entrust::EntrustView;
use crate::ui::position::PositionView;
use crate::ui::View;
use crate::ui::tree::{SubTree, Tree};

pub struct TradeView {
    strategy: Tree<String>,
    strategy_running: Tree<String>,
    path: String,

    module: Module,

    position_view: PositionView,
    entrust_view: EntrustView,
    account_view: AccountView,
}

impl TradeView {
    pub fn new(module: Module) -> Self {
        // let tree1 = Tree {
        //     name: "root".to_string(),
        //     selected: "".to_string(),
        //     sub_tree: vec![
        //         SubTree {
        //             name: "root/策略".to_string(),
        //             data: Some("hello2".to_string()),
        //             callback: Some(Box::new(|text| println!("callback_text: {}", text))),
        //             sub_tree: vec![],
        //         },
        //         SubTree {
        //             name: "root/策略2".to_string(),
        //             data: None,
        //             callback: None,
        //             sub_tree: vec![
        //                 SubTree::<String> {
        //                     name: "root/策略2/abc.py".to_string(),
        //                     data: Some("策略2".to_string()),
        //                     callback: None,
        //                     sub_tree: vec![],
        //                 }
        //             ],
        //         }, ],
        // };
        Self {
            strategy: Tree::new("策略"),
            strategy_running: Tree::new("策略运行中"),
            path: "".to_string(),

            module,
            position_view: PositionView::new(),
            entrust_view: EntrustView::new(),
            account_view: AccountView::new(),
        }
    }

    pub fn build_side_panel(&mut self, ctx: &Context, store: &mut Store) {
        egui::SidePanel::left("trade.strategy.left")
            .resizable(true)
            .show(ctx, |ui| {
                self.strategy.ui(ui);
                self.strategy_running.ui(ui);
                // self.tree2.ui(ui);
                // CollapsingHeader::new("策略").show(ui, |ui| {});
                // CollapsingHeader::new("策略(5运行中)").show(ui, |ui| {})
            });
    }
    pub fn build_strategy_setting(&mut self, ctx: &Context, store: &mut Store) {
        egui::SidePanel::left("trade.strategy.setting")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("策略设置");
                });
                ui.separator();

                // 资金
                Grid::new("trade.account.setting.fund")
                    .num_columns(4)
                    .spacing(Vec2::new(10.0, 10.0))
                    .show(ui, |ui| {

                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("交易类别:"));
                        ui.label("股票");

                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("运行模式:"));
                        ui.label("模拟");
                        ui.end_row();

                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("初始资金:"));
                        ui.label(format!("{:.4}", 10.0));
                        ui.end_row();

                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("券商手续费:"));
                        ui.label(format!("{:.4}", 0.024));
                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("过户费:"));
                        ui.label(format!("{:.4}", 100.0));
                        ui.end_row();

                        ui.with_layout(Layout::right_to_left(),
                                       |ui|
                                           ui.label("印花税:"));
                        ui.label(format!("{:.4}", 34.1));
                        ui.end_row();
                    });
                ui.separator();

                CollapsingHeader::new("策略参数").show(ui, |ui| {
                    Grid::new("trade.strategy.param")
                        .min_col_width(200.0)
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("参数名:");
                                ui.add(TextEdit::singleline(&mut self.path));
                            });
                            ui.horizontal(|ui| {
                                ui.label("参数值:");
                                ui.add(TextEdit::singleline(&mut self.path));
                            });
                            ui.end_row();
                        });
                    ui.separator();

                    if ui.button("+").clicked() {

                    }

                });
                CollapsingHeader::new("风控策略").show(ui, |ui| {

                });
                CollapsingHeader::new("券商接口").show(ui, |ui| {

                });
                ui.separator();

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        if ui.button("检测").clicked() {

                        }
                        if ui.button("应用").clicked() {

                        }
                        if ui.button("运行").clicked() {

                        }
                    })
                });

            });
    }
    fn strategy_path_build(&mut self, tree_path: &mut Vec<TreePath>, sub_tree: &mut Vec<SubTree<String>>) {
        for path in tree_path.iter_mut() {
            let mut t = SubTree::new(path.name.as_str());
            if !path.sub_path.is_empty() {
                self.strategy_path_build(&mut path.sub_path, &mut t.sub_tree);
            }
            sub_tree.push(t);
        }
    }
    pub fn strategy_rebuild(&mut self, store: &mut Store) {
        if !store.trade.strategy_is_build {
            self.strategy.reset(self.strategy.name.clone().as_str());
            let mut sub_tree: Vec<SubTree<String>> = vec![];
            self.strategy_path_build(&mut store.trade.strategy.sub_path,
                                     &mut sub_tree);
            self.strategy.sub_tree.clear();
            self.strategy.sub_tree.extend(sub_tree)
        }
    }
}

impl View for TradeView {
    fn show(&mut self, ctx: &Context, store: &mut Store) {
        egui::TopBottomPanel::bottom("trade.bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(store.trade.strategy_show, "策略").clicked() {
                    store.trade.strategy_show = !store.trade.strategy_show;
                }
                ui.separator();
                ui.selectable_label(false, "参数");
                ui.selectable_label(false, "持仓");
                ui.selectable_label(false, "委托");
                ui.selectable_label(false, "成交");
                ui.selectable_label(false, "信号");
                ui.selectable_label(false, "日志");

                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    for i in 0..=5 {
                        ui.selectable_label(false, format!("策略{}(+2.3%)", i));
                    }
                });
            })
        });


        if store.trade.strategy_show {
            self.build_side_panel(ctx, store);
            self.build_strategy_setting(ctx, store);
        }
        self.entrust_view.ui(ctx, &mut store.trade.account.entrust);
        self.position_view.ui(ctx, &mut store.trade.account.position);


        egui::TopBottomPanel::top("trade.top").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                // ComboBox::from_label("")
                //     .selected_text(format!("{}", self.path))
                //     .show_ui(ui, |ui| {
                //         ui.selectable_value(&mut self.path, "First".to_string(), "First");
                //         ui.selectable_value(&mut self.path, "Second".to_string(), "Second");
                //         ui.selectable_value(&mut self.path, "Third".to_string(), "Third");
                //     });
                ui.selectable_label(true, "测试策略(5)").on_hover_ui(|ui| {
                    ui.label("总共5个账户运行次策略:");
                    ui.label("神算：亏损: 100 -5.0%");
                    ui.label("神算2：亏损: 100 -5.0%");
                });
                if ui.selectable_label(true, "神算子(5)").clicked() {

                }

            });
        });

        self.account_view.ui(ctx, &mut store.trade.account);

    }
}

