use axum::{Router, Extension, routing::get};
use infra::deps::Deps;
use sqlx::postgres::PgPoolOptions;
use user::{infra::repositories::PgRepository as UserRepository, domain::presentation::controllers::index};
use crate::{infra::deps::Repositories};
use std::{sync::Arc, net::SocketAddr};

mod infra;
mod user;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new().max_connections(5).connect("").await.expect("failed to create postgres connection pool");

    let user_repository = UserRepository::new(pool);

    let repositories = Repositories::new(Arc::new(user_repository));

    let deps = Arc::new(Deps::new(repositories));

    let app = Router::new()
        .route("/users", get(index))
        .layer(Extension(deps));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
