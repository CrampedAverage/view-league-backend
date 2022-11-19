use actix_web::{get, Either, HttpResponse};

use crate::{services::champions::get_champions, types::Responder};

#[get("/champions")]
pub async fn champions() -> Responder {
    let result = get_champions().await;
    if result.is_ok() {
        let data = result.unwrap();
        Either::Left(HttpResponse::Ok().body(data))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
}
