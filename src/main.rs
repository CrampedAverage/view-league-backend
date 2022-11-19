use actix_web::{middleware, web, App, HttpServer};
use routes::{champion::champions, matches::match_by_id, summoner::summoner};

mod configuration;
mod lib;
mod routes;
mod services;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let port = str::parse(&std::env::var("PORT").unwrap_or("4000".to_string())).unwrap();

    HttpServer::new(move || {
        App::new().wrap(middleware::Compress::default()).service(
            web::scope("/api")
                .service(champions)
                .service(summoner)
                .service(match_by_id),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
