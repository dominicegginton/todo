use crate::item::Item;
use std::env;
use std::error::{self};

static FILE_NAME: &str = "todo.json";

pub enum Mode {
    Normal,
    Insert,
    Edit,
    Confirmation,
}

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub local_list: bool,
    pub mode: Mode,
    pub cursor_position: usize,
    pub input: String,
    pub selected_item: usize,
    pub items: Vec<Item>,
}

impl App {
    pub const fn new() -> Self {
        Self {
            running: true,
            local_list: false,
            mode: Mode::Normal,
            input: String::new(),
            cursor_position: 0,
            selected_item: 0,
            items: Vec::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_input(&mut self) {
        let input_string = self.input.trim().to_string();
        if input_string.is_empty() {
            return;
        }
        match self.mode {
            Mode::Edit => {
                let item = &mut self.items[self.selected_item];
                item.content = input_string;
                self.mode = Mode::Normal;
                self.input.clear();
                self.reset_cursor();
            }
            Mode::Insert => {
                let new_item = Item::new(input_string);
                self.items.insert(0, new_item);
                self.input.clear();
                self.reset_cursor();
            },
            _ => {}
        }
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

    pub fn toggle_selection_complete(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let item = &mut self.items[self.selected_item];
        item.complete = !item.complete;
    }

    pub fn edit_selected(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let item = &self.items[self.selected_item];
        self.input = item.content.clone();
        self.cursor_position = self.input.len();
        self.mode = Mode::Edit;
    }

    pub fn delete_selected(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.items.remove(self.selected_item);
        if self.selected_item >= self.items.len() {
            self.selected_item = self.items.len().saturating_sub(1);
        }
    }

    fn items_file(&self) -> String {
        match self.local_list {
            true => FILE_NAME.to_string(),
            false => {
                let home = env::var("HOME").clone().unwrap_or_default();
                if !home.is_empty() {
                    return format!("{}/{}", home, FILE_NAME);
                }
                FILE_NAME.to_string()
            }
        }
    }

    pub fn write_items_to_file(&self) {
        let serialized = serde_json::to_string(&self.items).unwrap();
        std::fs::write(self.items_file(), serialized).unwrap();
    }

    pub fn read_items_from_file(&mut self) {
        if let Ok(content) = std::fs::read_to_string(self.items_file()) {
            self.local_list = false;
            let items: Vec<Item> = serde_json::from_str(&content).unwrap();
            self.items = items;
        }
    }
}
