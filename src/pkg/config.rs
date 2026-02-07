use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server:ServerConfig,
    pub db:DBConfig,
    pub jwt:JwtConfig,
    pub middleware: Middleware,
}



#[derive(Debug,Deserialize)]
pub struct ServerConfig {
    pub host:String,
    pub port: u16,
}


#[derive(Debug,Deserialize)]
pub struct DBConfig {
    pub user: String,
    pub password: String, 
    pub host:String,
    pub port: u16,
    pub name:String,
}



#[derive(Debug,Deserialize)]
pub struct JwtConfig {
   pub secret: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Middleware {
    pub model_path: String, 
    pub policy_path: String
}




pub fn from_file() -> Config {
    let content = fs::read_to_string("config/config.yaml")
        .unwrap_or_else(|e| {
            eprintln!("Failed to read config file: {}", e);
            panic!();
        });

    serde_yaml::from_str(&content)
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse config yaml: {}", e);
            panic!();
        })
}