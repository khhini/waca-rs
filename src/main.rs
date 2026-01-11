use clap::{Parser, Subcommand};
use std::env;

use sqlx::SqlitePool;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { url: String, description: String },
    List,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add { url, description }) => {
            println!("Adding new bookmarks");
            let id = add(&pool, url, description).await?;
            println!("Added new bookmark id {id}");
        }
        Some(Command::List) => {
            list(&pool).await?;
        }
        None => {}
    }

    Ok(())
}

async fn list(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id,  url,description
FROM bookmarks
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!("- [{}]({})", &rec.description, &rec.url)
    }
    Ok(())
}

async fn add(pool: &SqlitePool, url: String, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
INSERT INTO bookmarks ( url, description)
VALUES ( ?1, ?2 )
        "#,
        url,
        description
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
