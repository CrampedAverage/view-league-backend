use crate::{services::champions::get_champions, types::Responder};
use actix_web::{get, middleware, web, App, Either, HttpResponse, HttpServer};
use routes::{matches::match_by_id, summoner::summoner};

mod configuration;
mod lib;
mod routes;
mod services;
mod types;

#[get("/champions")]
async fn champions() -> Responder {
    let result = get_champions().await;
    if result.is_ok() {
        let data = result.unwrap();
        Either::Left(HttpResponse::Ok().body(data))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
}

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
