/*
 * @Description: first web项目
 * @Author: didadida262
 * @Date: 2024-09-03 00:35:15
 * @LastEditors: didadida262
 * @LastEditTime: 2024-09-03 11:14:29
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
