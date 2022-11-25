#[cfg(feature = "api")]
mod blocks;
mod auth;

use crate::sync::DbPool;

use super::*;
use axum::{
    extract::{ws::Message, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use dashmap::DashMap;
use jwst::{parse_history, parse_history_client, Workspace};
use serde_json::Value as JsonValue;
use tokio::sync::{mpsc::Sender, Mutex};

#[derive(Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::IntoParams))]
pub struct Pagination {
    #[serde(default)]
    offset: usize,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    usize::MAX
}

#[derive(Serialize)]
pub struct PageData<T> {
    total: usize,
    data: T,
}

pub struct Context {
    pub workspace: DashMap<String, Mutex<Workspace>>,
    pub channel: DashMap<(String, String), Sender<Message>>,
    pub db: DbPool,
}

impl Context {
    pub async fn new(default_pool: Option<DbPool>) -> Self {
        Context {
            workspace: DashMap::new(),
            channel: DashMap::new(),
            db: default_pool.unwrap_or(
                DbPool::init_pool("jwst")
                    .await
                    .expect("Cannot create database"),
            ),
        }
    }
}

pub fn api_handler(router: Router) -> Router {
    let mut router = router;
    #[cfg(feature = "auth")]
    {
        router = auth::auth_apis(router);
    }  
    #[cfg(feature = "api")]
    {
        router = blocks::blocks_apis(router);
    }
    #[cfg(not(feature = "api"))]
    {
        router = router
    }
    router
}
