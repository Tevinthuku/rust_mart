use anyhow::anyhow;
use sqlx::postgres::PgPool;

pub async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!()
        .run(pool)
        .await
        .map_err(|err| anyhow!("Failed to run the migrations {err:?}"))
}
