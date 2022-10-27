use crate::{
    configuration::get_api_key,
    services::{champions::get_champions, summoner::get_summoner_data},
};
use actix_web::{get, middleware, web, App, Either, HttpResponse, HttpServer};
use serde::Deserialize;
use serde_json::json;

mod configuration;
mod lib;
mod services;
mod types;

type Responder = Either<HttpResponse, HttpResponse>;
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

#[derive(Deserialize, Debug)]
pub struct SummonerGetDataQuery {
    summoner_name: String,
    region: String,
    continent: String,
}

#[get("/summoner/get-data")]
async fn summoner(query: web::Query<SummonerGetDataQuery>) -> Responder {
    let api_key = get_api_key();
    println!("hehe");
    let result = get_summoner_data(query.into_inner(), &api_key).await;

    if result.is_ok() {
        let result_json = json!(result.unwrap()).to_string();
        Either::Left(HttpResponse::Ok().body(result_json))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Not FOUND"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let port = str::parse(&std::env::var("PORT").unwrap_or("4000".to_string())).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(web::scope("/api").service(champions).service(summoner))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
