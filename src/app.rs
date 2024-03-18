use crate::file::File;
use crate::input::Input;
use crate::item_list::ItemList;
use std::error::{self};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub file: File,
    pub input: Input,
    pub list: ItemList,
}

impl App {
    pub const fn new() -> Self {
        Self {
            running: true,
            file: File::new(),
            input: Input::new(),
            list: ItemList::new(),
        }
    }
}
