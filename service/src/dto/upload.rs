use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadRequest {
    pub dir: String,
    pub otp: String,
}
