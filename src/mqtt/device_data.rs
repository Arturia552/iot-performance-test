use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceData {
    pub timestamp: String,
    pub pub_reason: String,
    pub online: u8,
    pub data: Value,
}

impl DeviceData {
    pub fn set_timestamp(&mut self, timestamp: String) {
        self.timestamp = timestamp
    }

    pub fn get_data(&self) -> &Value {
        &self.data
    }
}
