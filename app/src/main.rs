use database::entity::user::User;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let db = database::connect(&std::env::var("DATABASE_URL")?, true).await?;

    if let Ok(user) = (User {
        id: None,
        username: "admin".to_string(),
        email: "admin@admin.com".to_string(),
    }
    .create(&db)
    .await)
    {
        println!("created new user: {:#?}", user);
    } else {
        println!("admin already exist");
    }

    let user = User::get(&db, 1).await;

    println!("{:#?}", user);

    Ok(())
}
