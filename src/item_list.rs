use crate::item::Item;

pub struct ItemList {
    pub items: Vec<Item>,
    pub selected_item: usize,
}

impl ItemList {
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_item: 0,
        }
    }

    pub fn add_item(&mut self, content: String) {
        let item = Item::new(content);
        self.items.insert(0, item);
    }

    pub fn move_selection_up(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.selected_item == self.items.len().saturating_sub(1) {
            return;
        }
        let new_selected_item = self.selected_item.saturating_add(1);
        self.selected_item = new_selected_item;
    }

    pub fn move_selection_down(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.selected_item == 0 {
            return;
        }
        let new_selected_item = self.selected_item.saturating_sub(1);
        self.selected_item = new_selected_item;
    }

    pub fn toggle_selected_item(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let item = &mut self.items[self.selected_item];
        item.complete = !item.complete;
    }

    pub fn edit_selected_item(&mut self, content: String) {
        if self.items.is_empty() {
            return;
        }
        let item = &mut self.items[self.selected_item];
        item.content = content;
    }

    pub fn remove_selected_item(&mut self) -> () {
        if self.items.is_empty() {
            return;
        }
        self.items.remove(self.selected_item);
        if self.selected_item >= self.items.len() {
            self.selected_item = self.items.len().saturating_sub(1);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_item = 0;
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected_item = index.unwrap_or(0);
    }

    pub fn get_selected_item(&self) -> Option<&Item> {
        if self.items.is_empty() {
            return None;
        }
        let item = &self.items[self.selected_item];
        Some(item)
    }
}
