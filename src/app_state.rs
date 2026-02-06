use std::{ sync::Arc};

use crate::{pkg::{ config::Config}, repo::{author::*, book::*, user::UserRepo}, services::jwt_service::JwtService};
use casbin::Enforcer;
use sqlx::MySqlPool;
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<Repo>,
    pub service: Arc<Service>,
}


pub struct Repo {
  pub book: BookRepo,
  pub author: AuthorRepo,
  pub user: UserRepo
}


pub struct Service {
    pub jwt: JwtService,
    pub enforcer: Enforcer,
}

impl AppState {
    pub fn new(db: MySqlPool, config: &Config, enforcer: Enforcer) -> Self {

        let repo = Arc::new(
            Repo{
                author: AuthorRepo::new(db.clone()),
                book: BookRepo::new(db.clone()),
                user: UserRepo::new(db.clone())
        
            }
        );
        let service = Arc::new(
            Service{
                jwt: JwtService::new(&config.jwt),
                enforcer: enforcer
            }
        );


        AppState { repo,  service}
    }
}
