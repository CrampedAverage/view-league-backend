use actix_web::{get, post, web, App, Either, HttpResponse, HttpServer, Responder};
mod services;

type ChampionsResult = Either<HttpResponse, HttpResponse>;
#[get("/champions")]
async fn hello() -> ChampionsResult {
    let result = services::champions::get_champions().await;
    if result.is_ok() {
        let data = result.unwrap();
        Either::Left(HttpResponse::Ok().body(data))
    } else {
        Either::Right(HttpResponse::BadRequest().body("Bad Request"))
    }
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
