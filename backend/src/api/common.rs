use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub cmd: String,
}

impl Message {
    pub fn from_str(s: &str) -> Message {
        serde_json::from_str(s).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
