use clap::{Parser, Subcommand};
use std::env;
use waca_rs::bookmark::BookmarkSqliteRepo;

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
    let sqlite_repo = BookmarkSqliteRepo::new(pool);

    match args.cmd {
        Some(Command::Add { url, description }) => {
            println!("Adding new bookmarks");
            let id = sqlite_repo.add(&url, &description).await?;
            println!("Added new bookmark id {id}");
        }
        Some(Command::List) => {
            let recs = sqlite_repo.list().await?;
            dbg!(recs);
        }
        None => {}
    }

    Ok(())
}
