use sqlx::PgPool;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
   let database_url = std::env::var("DATABASE_URL").map_err(|e | sqlx::Error::Configuration(e.into()))?;

   PgPool::connect(&database_url).await
}
