use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ChampionImage {
    full: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ChampionData {
    version: String,
    name: String,
    id: String,
    title: String,
    image: ChampionImage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseObject {
    data: HashMap<String, ChampionData>,
    version: String,
    format: String,
}

async fn dto_champions(champion_api_response: &str) -> Result<String> {
    let full_data: ResponseObject = serde_json::from_str(champion_api_response).unwrap();

    // dbg!(&full_data);
    let json_data = json!(&full_data);
    dbg!(json_data.to_string());
    println!("I WORK");
    Ok(json_data.to_string())
}

#[get("/champions")]
async fn hello() -> impl Responder {
    let body =
        reqwest::get("http://ddragon.leagueoflegends.com/cdn/11.8.1/data/en_US/champion.json")
            .await
            .unwrap()
            .text()
            .await;

    let data = body.unwrap();
    let result = dto_champions(&*data).await.unwrap();

    print!("hello world");
    HttpResponse::Ok().body(result)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::Compress::default())
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4040))?
    .run()
    .await
}
