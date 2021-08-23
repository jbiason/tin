use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;

async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("testing.sqlite")
                .create_if_missing(true),
        )
        .await?;
    sqlx::migrate("migrations").run(&pool).await?;
    Ok(pool)
}
