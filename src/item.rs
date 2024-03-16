use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub content: String,
    pub complete: bool,
}

impl Item {
    pub fn new(content: String) -> Item {
        Item {
            content,
            complete: false,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.complete {
            write!(f, "[x] {}", self.content)
        } else {
            write!(f, "[ ] {}", self.content)
        }
    }
}
