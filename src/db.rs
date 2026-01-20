use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::models::{
    GetEntriesRequest, GetEntriesResponse, GuestbookEntry, GuestbookEntryRequest, IdRequest,
    PubGuestbookEntry, SortOrder,
};

pub const DB_PATH: &str = "data/guestbook.sqlite";

pub async fn init_db() -> SqlitePool {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory for database");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:file:{}?mode=rwc", DB_PATH))
        .await
        .expect("Failed to open DB");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn insert_entry(
    pool: &SqlitePool,
    new: GuestbookEntryRequest,
) -> Result<GuestbookEntry, sqlx::Error> {
    let entry = GuestbookEntry {
        id: Uuid::new_v4().to_string(),
        name: new.name,
        email: new.email,
        rating: new.rating,
        note: new.note,
        posted_at_utc: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    sqlx::query(
        r#"
        INSERT INTO guestbook_entries
        (id, name, email, rating, note, posted_at_utc)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
    )
    .bind(entry.id.to_string())
    .bind(&entry.name)
    .bind(&entry.email)
    .bind(entry.rating)
    .bind(&entry.note)
    .bind(entry.posted_at_utc)
    .execute(pool)
    .await?;

    Ok(entry)
}

pub async fn get_entries(
    pool: &SqlitePool,
    get_params: GetEntriesRequest,
) -> Result<GetEntriesResponse, sqlx::Error> {
    let query = match get_params.sort {
        SortOrder::DatePostedDesc => {
            r#"
            SELECT id, name, rating, note, posted_at_utc
            FROM guestbook_entries
            ORDER BY posted_at_utc desc
        "#
        }
        SortOrder::DatePostedAsc => {
            r#"
            SELECT id, name, rating, note, posted_at_utc
            FROM guestbook_entries
            ORDER BY posted_at_utc asc
        "#
        }
        SortOrder::NameDesc => {
            r#"
            SELECT id, name, rating, note, posted_at_utc
            FROM guestbook_entries
            ORDER BY name desc
        "#
        }
        SortOrder::NameAsc => {
            r#"
            SELECT id, name, rating, note, posted_at_utc
            FROM guestbook_entries
            ORDER BY name asc
        "#
        }
    };

    let entries = sqlx::query_as::<_, PubGuestbookEntry>(&query)
        .fetch_all(pool)
        .await?;

    Ok(GetEntriesResponse { entries })
}

pub async fn entry_by_id(
    pool: &SqlitePool,
    entry_request: IdRequest,
) -> Result<PubGuestbookEntry, sqlx::Error> {
    let entry = sqlx::query_as::<_, PubGuestbookEntry>(
        r#"
        SELECT id, name, rating, note, posted_at_utc
        FROM guestbook_entries
        WHERE id = ?
        "#,
    )
    .bind(entry_request.id)
    .fetch_one(pool)
    .await?;

    Ok(entry)
}

pub async fn delete_entry(pool: &SqlitePool, entry_request: IdRequest) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM guestbook_entries
        WHERE id = ?
        "#,
    )
    .bind(entry_request.id)
    .execute(pool)
    .await?;

    Ok(())
}
