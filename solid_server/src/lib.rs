pub mod configuration;

use std::net::SocketAddr;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse, delete, get, head, options, patch, post, put};

use solid_storage;
use solid_auth;
use crate::configuration::Configuration;

#[actix_web::main]
pub async fn start_server(configuration: Configuration) -> std::io::Result<()> {
    let addr = SocketAddr::new(configuration.server.bind_address, configuration.server.port);

    println!("Runnning Solid Server on http://{}", addr);
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    solid_storage::init();

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
            .service(solid_auth::register)
            .service(solid_auth::login)
            .service(index)
            .service(handle_get)
            .service(handle_delete)
            .service(handle_get)
            .service(handle_head)
            .service(handle_options)
            .service(handle_patch)
            .service(handle_post)
            .service(handle_put)
    })
        .bind(&addr)?
        .run()
        .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Solid.rs!")
}

#[delete("/*")]
async fn handle_delete(req: HttpRequest) -> HttpResponse {
    // TODO: Implement behaviour for that method
    let path = req.path();
    HttpResponse::Ok().body(format!("Received delete request for {}", path))
}

#[get("/*")]
async fn handle_get(req: HttpRequest) -> HttpResponse {
    // TODO: Handle file not found
    // current behavior: returns empty body
    let path = req.path();
    HttpResponse::Ok().body(solid_storage::read_file(path).unwrap())
}

#[head("/*")]
async fn handle_head(req: HttpRequest) -> HttpResponse {
    // TODO: Implement behaviour for that method
    let path = req.path();
    HttpResponse::Ok().body(format!("Received head request for {}", path))
}

#[options("/*")]
async fn handle_options(req: HttpRequest) -> HttpResponse {
    // TODO: Implement behaviour for that method
    let path = req.path();
    HttpResponse::Ok().body(format!("Received options request for {}", path))
}

#[patch("/*")]
async fn handle_patch(req: HttpRequest) -> HttpResponse {
    // TODO: Implement behaviour for that method
    let path = req.path();
    HttpResponse::Ok().body(format!("Received patch request for {}", path))
}

#[post("/*")]
async fn handle_post(req: HttpRequest) -> HttpResponse {
    // TODO: Implement behaviour for that method
    let path = req.path();
    HttpResponse::Ok().body(format!("Received put request for {}", path))
}

#[put("/*")]
async fn handle_put(req: HttpRequest, bytes: web::Bytes) -> HttpResponse {
    match String::from_utf8(bytes.to_vec()) {
        Ok(content) => {
            match solid_storage::write_file(req.path(), content.as_str()) {
                Ok(()) => HttpResponse::Ok().body("success"),
                Err(error) => HttpResponse::InternalServerError().into()
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
 
}
