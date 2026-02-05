use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    pub id: u64,
    pub name:String,
    pub phone_number: String,
    pub address:String 
}



#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub name:String,
    pub phone_number:String,
    pub address:String
}




