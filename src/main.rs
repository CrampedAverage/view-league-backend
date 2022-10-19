use actix_web::{get, middleware, App, Either, HttpResponse, HttpServer};
mod services;

type ChampionsResult = Either<HttpResponse, HttpResponse>;
#[get("/champions")]
async fn champions() -> ChampionsResult {
    let result = services::champions::get_champions().await;
    if result.is_ok() {
        let data = result.unwrap();
        Either::Left(HttpResponse::Ok().body(data))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(champions)
    })
    .bind(("127.0.0.1", 4040))?
    .run()
    .await
}
