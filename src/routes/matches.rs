use actix_web::{get, web, Either, HttpResponse};
use serde_json::json;

use crate::{
    configuration::get_api_key, services::matches::get_match_info_by_id, types::Responder,
};

#[get("/match/{continent}/{match_id}")]
pub async fn match_by_id(path: web::Path<(String, String)>) -> Responder {
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
