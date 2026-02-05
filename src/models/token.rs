use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token:String,
    pub refresh_token:String,
}