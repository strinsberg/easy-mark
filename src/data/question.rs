use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Question {
    pub num: u32,
    pub part: u32,
    pub out_of: u32,
}
