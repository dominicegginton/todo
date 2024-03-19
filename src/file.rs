use crate::item::Item;
use std::{env, error, fs};

static FILE_NAME: &str = "todo.json";

pub enum FileMode {
    Local,
    Global,
}

pub struct File {
    pub mode: FileMode,
}

impl File {
    pub const fn new() -> Self {
        Self {
            mode: FileMode::Global,
        }
    }

    pub fn read_items_from_file(&self) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let file = fs::read_to_string(self.file_path());
        if file.is_err() {
            return Ok(Vec::new());
        }
        let items: Vec<Item> = serde_json::from_str(&file.unwrap_or_default())?;
        Ok(items)
    }

    pub fn write_items_to_file(&self, items: &Vec<Item>) -> Result<(), Box<dyn error::Error>> {
        let items_json = serde_json::to_string(&items)?;
        fs::write(self.file_path(), items_json)?;
        Ok(())
    }

    fn file_path(&self) -> String {
        match self.mode {
            FileMode::Local => format!("./{}", FILE_NAME),
            FileMode::Global => format!(
                "{}/{}",
                match env::var("HOME") {
                    Ok(val) => val,
                    Err(_) => "".to_string(),
                },
                FILE_NAME
            ),
        }
    }
}
