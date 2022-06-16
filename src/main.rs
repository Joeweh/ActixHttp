use std::env;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server Online!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let HOST: String = env::var("HOST").expect("Host Not Set");
    let PORT: String = env::var("PORT").expect("Port Not Set");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}