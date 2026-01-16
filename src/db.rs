use crate::error::TodoError;
use sqlx::postgres::{ PgPool, PgPoolOptions };
use std::env;

pub async fn connect_to_db() -> Result<PgPool, TodoError>
{
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str()).await?;

    Ok(pool)
}
