use crate::services::{champions::get_champions, player::get_summoner_data};
use actix_web::{
    get, middleware,
    web::{self, Data},
    App, Either, HttpResponse, HttpServer,
};
use serde::Deserialize;
mod services;

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

#[get("/summoner/{region}/{name}")]
async fn summoner(
    path: web::Path<(String, String)>,
    config: web::Data<Configuration>,
) -> Responder {
    let found = false;
    let (region, name) = path.into_inner();
    let api_key = &config.riot_api_key;

    let _result = get_summoner_data(region, name, api_key).await;

    if found {
        Either::Left(HttpResponse::Ok().body("FOUND"))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Not FOUND"))
    }
}
#[derive(Deserialize, Debug, Clone)]
struct Configuration {
    port: u16,
    riot_api_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    match envy::from_env::<Configuration>() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Couldn't read config ({})", e),
    };
    let config = envy::from_env::<Configuration>().unwrap();
    let port = config.port;

    HttpServer::new(move || {
        App::new().wrap(middleware::Compress::default()).service(
            web::scope("/api")
                .app_data(Data::new(config.clone()))
                .service(champions)
                .service(summoner),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
