use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub ordinal: usize,
    pub name: String,
}

