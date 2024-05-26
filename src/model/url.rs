use anyhow::Result;
use sqlx::{FromRow, PgPool};

const ID_LENGTH: usize = 6;

#[derive(Debug, FromRow)]
pub struct Url {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}

pub async fn insert_url(db: &PgPool, url: impl Into<String>) -> Result<String> {
    let url = url.into();

    loop {
        let id = nanoid::nanoid!(ID_LENGTH);
        let resp = _insert_url(db, &id, &url).await;

        match resp {
            Ok(url) => return Ok(url.id),
            Err(_e) => continue,
        }
    }
}

pub async fn get_url(db: &PgPool, id: impl Into<String>) -> Result<Option<String>> {
    let sql = r#"
    SELECT url FROM urls WHERE id = $1
    "#;

    let id = id.into();
    let resp = sqlx::query_as(sql).bind(&id).fetch_optional(db).await?;
    Ok(resp.map(|url: Url| url.url))
}

async fn _insert_url(db: &PgPool, id: &String, url: &String) -> Result<Url> {
    let sql = r#"
    INSERT INTO urls (id, url) VALUES ($1, $2)
    ON CONFLICT (url) DO UPDATE SET url = EXCLUDED.url
    RETURNING id
    "#;

    sqlx::query_as(sql)
        .bind(id)
        .bind(url)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}
