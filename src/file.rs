use crate::item::Item;
use home;
use std::env;
use std::error;
use std::fs;

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
        let file = match self.mode {
            FileMode::Local => {
                let mut path = env::current_dir()?;
                path.push(FILE_NAME);
                fs::read_to_string(path)?
            }
            FileMode::Global => match home::home_dir() {
                Some(path) => {
                    let mut path = path;
                    path.push(FILE_NAME);
                    fs::read_to_string(path)?
                }
                None => return Err("Could not find home directory".into()),
            },
        };
        let items: Vec<Item> = serde_json::from_str(&file)?;
        Ok(items)
    }

    pub fn write_items_to_file(&self, items: &Vec<Item>) -> Result<(), Box<dyn error::Error>> {
        let file = serde_json::to_string_pretty(items)?;
        match self.mode {
            FileMode::Local => {
                let mut path = env::current_dir()?;
                path.push(FILE_NAME);
                fs::write(path, file)?;
            }
            FileMode::Global => match home::home_dir() {
                Some(path) => {
                    let mut path = path;
                    path.push(FILE_NAME);
                    fs::write(path, file)?;
                }
                None => return Err("Could not find home directory".into()),
            },
        }
        Ok(())
    }
}
