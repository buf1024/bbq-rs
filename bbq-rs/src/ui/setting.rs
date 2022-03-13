use std::borrow::BorrowMut;
use eframe::egui::{CollapsingHeader, Color32, Context, Grid, RichText, Spinner, TextEdit, Ui, Window};
use crate::store::{Settings, Store};
use crate::ui::{set_window_open, View};

#[derive(Debug)]
pub struct SettingView {
    // store.settings: &'a mut Settings,

    setting_widget: &'static str,
    orig_db_url: String,
}

impl SettingView {
    pub fn new() -> Self {
        Self {
            setting_widget: "setting.widget",
            orig_db_url: "".to_string(),
        }
    }
    fn ui(&mut self, ui: &mut Ui, store: &mut Store) {
        CollapsingHeader::new("数据库")
            .show(ui, |ui| {
                Grid::new("settings.email")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("数据连接串: ");
                        ui.add(TextEdit::singleline(&mut store.settings.db_url)
                            .hint_text("输入数据库连接串")
                        );
                        ui.end_row();

                        if store.settings.db_is_testing {
                            ui.add(Spinner::new());
                            ui.label("连接测试中...");
                        } else {
                            if ui.button("测试连接")
                                .clicked() {
                                store.settings.db_is_testing = true;
                            }

                            if self.orig_db_url != store.settings.db_url {
                                self.orig_db_url = store.settings.db_url.clone();
                                store.settings.db_is_valid = false;
                            }
                            if store.settings.db_is_valid {
                                ui.label(RichText::new("连接成功!").color(Color32::LIGHT_GREEN));
                            } else {
                                ui.label(RichText::new("连接无效!"));
                            }
                        }
                        ui.end_row();
                    });
            });
        CollapsingHeader::new("通知").show(ui, |ui| {
            CollapsingHeader::new("邮箱").show(ui, |ui| {
                Grid::new("store.settings.email")
                    .num_columns(2)
                    .show(ui, |ui| {

                        ui.label("发送地址(IP:PORT):");
                        ui.add(TextEdit::singleline(&mut store.settings.email_push.smtp_host));
                        ui.end_row();

                        ui.label("发送E-Mail:");
                        ui.add(TextEdit::singleline(&mut store.settings.email_push.user));
                        ui.end_row();

                        ui.label("发送E-Mail密钥:");
                        ui.add(TextEdit::singleline(&mut store.settings.email_push.secret));
                        ui.end_row();

                        ui.label("接收E-Mail:");
                        ui.add(TextEdit::singleline(&mut store.settings.email_push.notify));
                        ui.end_row();
                    });
            });

            CollapsingHeader::new("微信").show(ui, |ui| {
                Grid::new("store.settings.wechat")
                    .num_columns(2)
                    .show(ui, |ui| {

                        ui.label("Token:");
                        ui.add(TextEdit::singleline(&mut store.settings.wechat_push.token));
                        ui.end_row();
                    });
            });
        });

        CollapsingHeader::new("路径").show(ui, |ui| {
            CollapsingHeader::new("券商接口").show(ui, |ui| {

                for path in store.settings.broker_path.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(path));
                        if path.is_empty() || !std::path::Path::new(path).is_dir() {
                            ui.label(RichText::new("无效").color(Color32::LIGHT_RED));
                        } else {
                            ui.label(RichText::new("有效").color(Color32::LIGHT_GREEN));
                        }
                        if ui.button("浏览").clicked() {}
                    });
                }
                ui.separator();
                if ui.button("新增").clicked() {
                    store.settings.broker_path.push(String::new())
                }
            });

            CollapsingHeader::new("交易策略").show(ui, |ui| {
                for path in store.settings.strategy_path.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(path));
                        if path.is_empty() || !std::path::Path::new(path).is_dir() {
                            ui.label(RichText::new("无效").color(Color32::LIGHT_RED));
                        } else {
                            ui.label(RichText::new("有效").color(Color32::LIGHT_GREEN));
                        }
                        if ui.button("浏览").clicked() {}
                    });
                }
                ui.separator();
                if ui.button("新增").clicked() {
                    store.settings.strategy_path.push(String::new())
                }
            });

            CollapsingHeader::new("风控策略").show(ui, |ui| {

                for path in store.settings.risk_path.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(path));
                        if path.is_empty() || !std::path::Path::new(path).is_dir() {
                            ui.label(RichText::new("无效").color(Color32::LIGHT_RED));
                        } else {
                            ui.label(RichText::new("有效").color(Color32::LIGHT_GREEN));
                        }
                        if ui.button("浏览").clicked() {}
                    });
                }
                ui.separator();
                if ui.button("新增").clicked() {
                    store.settings.risk_path.push(String::new())
                }
            });
        });
    }
}

impl View for SettingView {
    fn show(&mut self, ctx: &Context, mut store: &mut Store) {
        set_window_open(&mut store.settings.open_windows, self.setting_widget, true);
        let mut open = true;
        Window::new("设置")
            .min_width(400.0)
            .vscroll(true)
            .hscroll(true)
            .open(&mut open)
            .show(ctx, |ui| {
                self.ui(ui, store)
            });
        store.settings.show = open;
        set_window_open(&mut store.settings.open_windows, self.setting_widget, open);
    }
}
