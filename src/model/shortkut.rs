use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShortKut {
    pub shortcuts: Vec<ShortKutType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShortKutType {
    pub alias: String,
    pub description: Option<String>,
    pub command: Value,
}
