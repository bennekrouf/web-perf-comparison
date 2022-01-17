use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on 0.0.0.0 port 80");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}