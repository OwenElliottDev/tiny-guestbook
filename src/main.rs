use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, Result, get, web};

mod auth;
mod db;
mod handlers;
mod models;

#[get("/sign")]
async fn sign() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/sign.html")?)
}

#[get("/theme.css")]
async fn theme() -> Result<NamedFile> {
    let name = std::env::var("THEME").unwrap_or_default();
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let name = if valid { name.as_str() } else { "default" };
    let file = NamedFile::open(format!("./static/css/themes/{name}.css"))
        .or_else(|_| NamedFile::open("./static/css/themes/default.css"))?;
    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::init_db().await;

    let api_key = std::env::var("GUESTBOOK_SECRET_KEY")
        .ok()
        .filter(|k| !k.is_empty());
    if api_key.is_none() {
        eprintln!("DELETE auth disabled: GUESTBOOK_SECRET_KEY not set");
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(models::ApiKey(api_key.clone())))
            .route("/api/health", web::get().to(handlers::health))
            .route("/api/sign", web::post().to(handlers::sign_guestbook))
            .route(
                "/api/entries",
                web::get().to(handlers::get_guestbook_entries),
            )
            .route("/api/entry", web::get().to(handlers::get_entry_by_id))
            .route("/api/entry", web::delete().to(handlers::delete_entry_by_id))
            .service(sign)
            .service(theme)
            .service(Files::new("/static", "./static"))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
