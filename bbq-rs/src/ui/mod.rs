use std::collections::BTreeSet;
use crate::Store;

pub mod app;

mod setting;
mod trade;
mod tree;
mod position;
mod entrust;
mod account;

pub trait View {
    fn show(&mut self, ctx: &eframe::egui::Context, store: &mut Store);
}

pub fn set_window_open(tree: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !tree.contains(key) {
            tree.insert(key.to_owned());
        }
    } else {
        tree.remove(key);
    }
}
