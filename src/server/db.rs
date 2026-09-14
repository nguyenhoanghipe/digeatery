use sqlx::PgPool;
use tokio::sync::OnceCell;

static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn connect_db() -> Result<PgPool, sqlx::Error> {
   let database_url = std::env::var("DATABASE_URL").map_err(|e | sqlx::Error::Configuration(e.into()))?;

   PgPool::connect(&database_url).await
}

pub async fn get_db() -> Result<&'static PgPool, sqlx::Error> {
    DB.get_or_try_init(connect_db).await
}
