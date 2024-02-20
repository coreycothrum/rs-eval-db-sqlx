use sqlx::sqlite::Sqlite;
use sqlx::Pool;

#[derive(Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
}

impl User {
    pub async fn get(db: &Pool<Sqlite>, id: i32) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(User, "select * from user where id = ?", id)
            .fetch_one(db)
            .await?)
    }

    pub async fn create(self, db: &Pool<Sqlite>) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            User,
            "INSERT INTO user (username, email) VALUES ($1, $2) RETURNING *",
            self.username,
            self.email
        )
        .fetch_one(db)
        .await?)
    }
}
