use sqlx::sqlite::{Sqlite, SqlitePoolOptions};
use sqlx::Pool;

pub mod entity;

pub async fn connect(uri: &str, migrate: bool) -> Result<Pool<Sqlite>, sqlx::Error> {
    let db = SqlitePoolOptions::default().connect(uri).await?;

    if migrate {
        sqlx::migrate!("./migrations").run(&db).await?;
    }

    Ok(db)
}
