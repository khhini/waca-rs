use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub description: String,
}

pub struct BookmarkSqliteRepo {
    pool: SqlitePool,
}

impl BookmarkSqliteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        BookmarkSqliteRepo { pool }
    }

    pub async fn list(self) -> anyhow::Result<Vec<Bookmark>> {
        let recs = sqlx::query_as!(
            Bookmark,
            r#"
SELECT id, url, description
FROM bookmarks
ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    pub async fn find_by_id(self, id: &i64) -> anyhow::Result<Bookmark> {
        let recs = sqlx::query_as!(
            Bookmark,
            r#"
SELECT id, url, description
FROM bookmarks
WHERE id = ?1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(recs)
    }

    pub async fn add(self, url: &str, description: &str) -> anyhow::Result<i64> {
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

    pub async fn update(self, id: &i64, url: &str, description: &str) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await?;

        let res = sqlx::query!(
            r#"
UPDATE bookmarks
SET url = ?2,
    description = ?3
WHERE id = ?1
            "#,
            id,
            url,
            description
        )
        .execute(&mut *conn)
        .await?
        .rows_affected();

        Ok(res)
    }

    pub async fn delete(self, id: &i64) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await?;

        let res = sqlx::query!(
            r#"
DELETE FROM bookmarks
WHERE id = ?1
            "#,
            id,
        )
        .execute(&mut *conn)
        .await?
        .rows_affected();

        Ok(res)
    }
}
