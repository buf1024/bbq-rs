use eframe::egui::{CollapsingHeader, Color32, RichText, Ui, Widget};

pub struct Tree<T: Clone> {
    pub name: String,
    pub selected: String,
    pub sub_tree: Vec<SubTree<T>>,
}

impl<T: Clone> Tree<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            selected: "".to_string(),
            sub_tree: vec![]
        }
    }
    pub fn reset(&mut self, name: &str) {
        self.name = name.to_string();
        self.selected = "".to_string();
        self.sub_tree.clear();
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        CollapsingHeader::new(self.name.as_str())
            .show(ui, |ui| {
                for sub_tree in self.sub_tree.iter_mut() {
                    sub_tree.ui(ui, &mut self.selected);
                }
            });
    }
}

pub struct SubTree<T: Clone> {
    pub name: String,
    pub data: Option<T>,
    pub callback: Option<Box<dyn Fn(&T) -> ()>>,
    pub sub_tree: Vec<SubTree<T>>,
}

impl<T: Clone> SubTree<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: None,
            sub_tree: vec![],
            callback: None
        }
    }
    pub fn ui(&mut self,ui: &mut Ui,selected: &mut String) {
        let text = self.name.split("/").last().unwrap();
        if self.data.is_some() {
            if ui.selectable_label(selected.as_str() == self.name.as_str(), &text[..]).clicked() {
                *selected = self.name.clone();
                if self.callback.is_some() {
                    self.callback.as_ref().unwrap()(self.data.as_ref().unwrap());
                }
            }
        }
        if !self.sub_tree.is_empty() {
            CollapsingHeader::new(&text[..])
                .show(ui, |ui| {
                    for sub_tree in self.sub_tree.iter_mut() {
                        sub_tree.ui(ui, selected);
                    }
                });
        }
    }
}

