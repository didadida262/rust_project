use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

//表单格式，对应 Html 中的 name
#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
}

//显示初始（pages/index.html）页面
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")//格式
        .body(include_str!("./page/index.html"))//读取文件作为相应主体
}

async fn post(p: web::Form<Person>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")//防止乱码
        .body(format!("欢迎 {}", p.name))//读取表单内容
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //以另一种方式实现 get 和 post 请求
            .service(web::resource("/").route(web::get().to(index)))//路由根目录
            .service(web::resource("/post").route(web::post().to(post)))//路由 /post 目录
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
