use serde::{Deserialize, Serialize};

///general APi response for both success and error
#[derive(Debug, Deserialize, Serialize)]
pub struct SeverResponse<T> {
    pub success: bool,
    message: String,
    data: Option<T>,
}

///Login data structure
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    #[serde(rename = "type")]
    pub token_type: String,
}
