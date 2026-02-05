use sqlx::MySqlPool;
use crate::pkg::config::Config;


pub async fn connect(cfg: &Config) -> Result< MySqlPool, sqlx::Error> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        cfg.db.user,
        cfg.db.password,
        cfg.db.host,
        cfg.db.port,
        cfg.db.name,
    );

    MySqlPool::connect(&url).await
}