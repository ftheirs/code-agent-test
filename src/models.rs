/ src/models.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyModel {
    pub id: i32,
    pub name: String,
}
