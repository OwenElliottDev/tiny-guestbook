use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GuestbookEntry {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub rating: i32,
    pub note: String,
    pub posted_at_utc: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PubGuestbookEntry {
    pub id: String,
    pub name: String,
    pub rating: i32,
    pub note: String,
    pub posted_at_utc: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestbookEntryRequest {
    pub name: String,
    pub email: Option<String>,
    pub rating: i32,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
    DatePostedDesc,
    DatePostedAsc,
    NameDesc,
    NameAsc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntriesRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub sort: SortOrder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntriesResponse {
    pub entries: Vec<PubGuestbookEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdRequest {
    pub id: String,
}
