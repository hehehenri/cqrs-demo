use std::sync::Arc;
use axum::Extension;
use axum_macros::debug_handler;
use crate::infra::deps::Deps;

#[debug_handler]
pub async fn index(Extension(state): Extension<Arc<Deps>>) {

}
