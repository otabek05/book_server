use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Author {
    pub id: i64,
    pub name:String
}

#[derive(Deserialize)]
pub struct AuthorPayload {
   pub name:String
}


