use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    pub id: u64,
    pub name:String,
    pub email: String,
    pub address:String,
    pub created_at: NaiveDateTime
}



#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name:String,
    pub email:String,
    pub address:String
}




