use actix_web::{
    get, middleware,
    web::{self},
    App, Either, HttpResponse, HttpServer,
};
use serde::Deserialize;
mod services;

type Responder = Either<HttpResponse, HttpResponse>;
#[get("/champions")]
async fn champions() -> Responder {
    let result = services::champions::get_champions().await;
    if result.is_ok() {
        let data = result.unwrap();
        Either::Left(HttpResponse::Ok().body(data))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
}

#[get("/summoner/{name}")]
async fn player(config: web::Data<Configuration>) -> Responder {
    let found = false;
    println!("Adnan: {}", config.riot_api_key);
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
    // env::var(key)
    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .wrap(middleware::Compress::default())
            .service(web::scope("/api").service(champions).service(player))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
