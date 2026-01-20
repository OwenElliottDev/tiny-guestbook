use actix_web::{HttpResponse, web};
use sqlx::SqlitePool;

use crate::db::{delete_entry, entry_by_id, get_entries, insert_entry};
use crate::models::{GetEntriesRequest, GuestbookEntryRequest, IdRequest};

pub async fn sign_guestbook(
    pool: web::Data<SqlitePool>,
    payload: web::Json<GuestbookEntryRequest>,
) -> HttpResponse {
    match insert_entry(&pool, payload.into_inner()).await {
        Ok(entry) => HttpResponse::Created().json(entry),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_guestbook_entries(
    pool: web::Data<SqlitePool>,
    payload: web::Query<GetEntriesRequest>,
) -> HttpResponse {
    match get_entries(&pool, payload.into_inner()).await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_entry_by_id(
    pool: web::Data<SqlitePool>,
    payload: web::Query<IdRequest>,
) -> HttpResponse {
    match entry_by_id(&pool, payload.into_inner()).await {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_entry_by_id(
    pool: web::Data<SqlitePool>,
    payload: web::Query<IdRequest>,
) -> HttpResponse {
    match delete_entry(&pool, payload.into_inner()).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
