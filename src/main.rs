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
    let sqlite_repo = BookmarkSqliteRepo::new(pool);

    match args.cmd {
        Some(Command::Add { url, description }) => {
            println!("Adding new bookmarks");
            let id = sqlite_repo.add(url, description).await?;
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

#[derive(Debug)]
struct Bookmark {
    id: i64,
    url: String,
    description: String,
}

struct BookmarkSqliteRepo {
    pool: SqlitePool,
}

impl BookmarkSqliteRepo {
    fn new(pool: SqlitePool) -> Self {
        BookmarkSqliteRepo { pool }
    }

    async fn list(self) -> anyhow::Result<Vec<Bookmark>> {
        let recs = sqlx::query_as!(
            Bookmark,
            r#"
SELECT id,  url,description
FROM bookmarks
ORDER BY id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }
    async fn add(self, url: String, description: String) -> anyhow::Result<i64> {
        let mut conn = self.pool.acquire().await?;

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
}
