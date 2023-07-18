use std::env;
use std::ops::Deref;
use std::sync::Arc;

use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::FromRequest;
use rocket::{request, Request, State};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::conf::Config;

use crate::resp::Error;

#[derive(Clone, Debug)]
pub struct Databases {
    pub default: Pool,
}

impl Databases {
    pub async fn new(config: Config) -> Self {
        println!("Databases init");
        let db = Databases {
            default: connect(config).await,
        };
        println!("{db:?}");
        db
    }
}

pub type Pool = PgPool;

pub async fn connect(config: Config) -> Pool {
    PgPoolOptions::new()
        .min_connections(10)
        .max_connections(config.max_size)
        .connect(config.database_url.as_str())
        .await
        .expect("Failed to create database pool")
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Databases {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Error> {
        let db = request.rocket().state::<Databases>();
        match db {
            Some(db) => request::Outcome::Success(db),
            None => request::Outcome::Failure((
                Status::ServiceUnavailable,
                Error::DatabaseConnectionError("从 State 获取连接池失败".to_string()),
            )),
        }
    }
}
