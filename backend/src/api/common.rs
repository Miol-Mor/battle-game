use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub cmd: String,
}

impl Message {
    pub fn new(cmd: &str) -> Message {
        Message {
            cmd: cmd.to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Message {
        serde_json::from_str(s).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
