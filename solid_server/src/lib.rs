pub mod configuration;

use std::convert::Infallible;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

use configuration::Configuration;

pub async fn start_server(configuration: &Configuration) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), configuration.port);

    let solid_svc = make_service_fn(|_| async { Ok::<_,hyper::Error>(service_fn(serve_solid)) });

    let server = Server::bind(&addr).serve(solid_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn serve_solid(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::DELETE => {
            *response.body_mut() = Body::from("Received DELETE request");
        },
        &Method::GET => {
            *response.body_mut() = Body::from("Received GET request");
        },
        &Method::HEAD => {
            *response.body_mut() = Body::from("Received HEAD request");
        },
        &Method::OPTIONS => {
            *response.body_mut() = Body::from("Received OPTIONS request");
        },
        &Method::PATCH => {
            *response.body_mut() = Body::from("Received PATCH request");
        },
        &Method::POST => {
            *response.body_mut() = Body::from("Received POST request");
        },
        &Method::PUT => {
            *response.body_mut() = Body::from("Received PUT request");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}
