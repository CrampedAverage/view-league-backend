use actix_web::{get, web, Either, HttpResponse};
use serde_json::json;

use crate::{
    configuration::get_api_key,
    services::summoner::get_summoner_data,
    types::{Responder, SummonerGetDataQuery},
};

#[get("/summoner/get-data")]
pub async fn summoner(query: web::Query<SummonerGetDataQuery>) -> Responder {
    let api_key = get_api_key();
    println!("hehe");
    let summoner_data_result = get_summoner_data(query.into_inner(), &api_key).await;
    if summoner_data_result.is_ok() {
        let summoner_data = summoner_data_result.unwrap();
        let response = json!(summoner_data).to_string();
        Either::Left(HttpResponse::Ok().body(response))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Not FOUND"))
    }
}
