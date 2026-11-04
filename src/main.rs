use actix_files::{Files, NamedFile};
use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer, Result, get, web};

mod auth;
mod db;
mod handlers;
mod models;

#[get("/sign")]
async fn sign(dir: web::Data<models::StaticDir>) -> Result<NamedFile> {
    Ok(NamedFile::open(format!("{}/sign.html", dir.0))?)
}

#[get("/theme.css")]
async fn theme(dir: web::Data<models::StaticDir>) -> Result<NamedFile> {
    let name = std::env::var("THEME").unwrap_or_default();
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let name = if valid { name.as_str() } else { "default" };
    let themes = format!("{}/css/themes", dir.0);
    let file = NamedFile::open(format!("{themes}/{name}.css"))
        .or_else(|_| NamedFile::open(format!("{themes}/default.css")))?;
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

    let static_dir = std::env::var("GUESTBOOK_STATIC_DIR").unwrap_or_else(|_| "static".into());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(models::ApiKey(api_key.clone())))
            .app_data(web::Data::new(models::StaticDir(static_dir.clone())))
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
            .service(Files::new("/static", &static_dir))
            .service(Files::new("/", &static_dir).index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
