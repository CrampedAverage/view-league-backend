use crate::{
    configuration::get_api_key,
    services::{champions::get_champions, matches::get_match_info_by_id},
    types::Responder,
};
use actix_web::{get, middleware, web, App, Either, HttpResponse, HttpServer};
use routes::summoner::summoner;
use serde_json::json;

mod configuration;
mod lib;
mod routes;
mod services;
mod types;

#[get("/match/{continent}/{match_id}")]
async fn match_by_id(path: web::Path<(String, String)>) -> Responder {
    let api_key = get_api_key();
    let (continent, match_id) = path.into_inner();
    println!("{}", match_id);
    let result = get_match_info_by_id(api_key, match_id, continent).await;
    if result.is_ok() {
        let data = result.unwrap();
        let response = json!(data).to_string();
        Either::Left(HttpResponse::Ok().body(response))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
}

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
