use sqlx::MySqlPool;
use crate::config::Config;

pub async fn create_pool(cfg: &Config) -> MySqlPool {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        cfg.db.user,
        cfg.db.password,
        cfg.db.host,
        cfg.db.port,
        cfg.db.name,
    );

    MySqlPool::connect(&url).await.expect("DB connect failed")
}



