#[macro_use]
extern crate diesel;

use actix_web::{
    dev::ServiceRequest,
    web,
    App,
    Error,
    HttpServer,
};
use actix_web_httpauth::{
    extractors::{
        AuthenticationError,
        bearer::{BearerAuth, Config}
    },
    middleware::HttpAuthentication,
};
use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager,
    },
};
use anyhow::Context;

mod errors;
mod handlers;
mod models;
mod schema;
mod auth;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL")
        .with_context(|| format!("DATABASE_URL must be set."))?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .with_context(|| format!("Failed to create database connection pool."))?;

    let factory = move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    };
    HttpServer::new(factory)
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) if res => Ok(req),
        _ => Err(AuthenticationError::from(config).into()),
    }
}