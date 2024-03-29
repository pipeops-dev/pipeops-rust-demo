use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "<h1>Welcome to Rust on PipeOps!</h1>"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}