use serde::Deserialize;
use std::{env, fs};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub server:ServerConfig,
    pub db:DBConfig,
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



pub fn from_file() -> Config{
      let cwd = env::current_dir().unwrap();
    println!("Current working directory: {}", cwd.display());
    let content = fs::read_to_string("config/config.yaml").expect("config yaml not found");
    serde_yaml::from_str(&content).expect("invalid config.yaml")
}