use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, Result, get, web};

mod db;
mod handlers;
mod models;

#[get("/sign")]
async fn sign() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/sign.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/sign", web::post().to(handlers::sign_guestbook))
            .route(
                "/api/entries",
                web::get().to(handlers::get_guestbook_entries),
            )
            .route("/api/entry", web::get().to(handlers::get_entry_by_id))
            .route("/api/entry", web::delete().to(handlers::delete_entry_by_id))
            .service(sign)
            .service(Files::new("/static", "./static"))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
