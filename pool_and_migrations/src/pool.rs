use anyhow::Context;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Pool {
    pub(crate) pg_pool: Arc<PgPool>,
}

impl Pool {
    pub async fn new(max_connections: u32) -> anyhow::Result<Self> {
        let db_env = env::var("DATABASE_URL").context("Failed to get DATABASE_URL from env")?;
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_env)
            .await?;
        Ok(Self {
            pg_pool: Arc::new(pool),
        })
    }

    pub fn get(&self) -> &PgPool {
        &self.pg_pool
    }
}
