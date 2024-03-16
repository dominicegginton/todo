use serde::{Deserialize, Serialize};

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
