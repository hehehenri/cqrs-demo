use infra::state::State;
use sqlx::postgres::PgPoolOptions;
use user::infra::repositories::PgRepository as UserRepository;
use crate::{infra::state::Repositories};
use std::sync::Arc;

mod infra;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new().max_connections(5).connect("").await?;

    let user_repository = UserRepository::new(pool);

    let repositories = Repositories::new(Arc::new(user_repository));

    let state = State::new(repositories);

    Ok(())
}
