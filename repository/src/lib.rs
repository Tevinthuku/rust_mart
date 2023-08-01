pub mod cart;
pub mod errors;
pub mod pool;
pub mod product;
pub mod schema;

use std::env;

use anyhow::Context;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use errors::RepositoryError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .context("Connection pool could not be built")?;
        Ok(Repository { pool })
    }

    fn conn(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, RepositoryError> {
        self.pool
            .get()
            .context("Failed to get connection")
            .map_err(RepositoryError::new_internal)
    }
}
