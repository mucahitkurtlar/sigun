use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewOtpRequest {
    pub secret: String,
}
