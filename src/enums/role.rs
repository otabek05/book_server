

pub enum Role {
    Admin,
    User
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}