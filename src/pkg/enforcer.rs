
use casbin::{CoreApi, DefaultModel, Enforcer, FileAdapter};
use crate::pkg::{config::Middleware};


pub async fn new(middleware: Middleware) -> Result < Enforcer, casbin::Error> {
    let model = DefaultModel::from_file(middleware.model_path).await?;
    let adapter = FileAdapter::new(middleware.policy_path);
    let mut enforcer  = Enforcer::new(
       model, adapter
    ).await?;

    enforcer.load_policy().await?;
    Ok(enforcer)
}