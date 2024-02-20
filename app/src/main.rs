#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let _db = database::connect(&std::env::var("DATABASE_URL")?, true).await?;

    Ok(())
}
