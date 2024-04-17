use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub token: String,
    pub path: String,
}
