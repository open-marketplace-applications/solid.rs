use identity_core::did::{Param, DID};
use actix_web::{HttpRequest, HttpResponse, post};

fn main() {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into(), "789011".into()],
        params: Some(vec![Param::from(("name".into(), Some("value".into())))]),
        ..Default::default()
    }
    .init()
    .unwrap();

    println!("WHAT DO FU");
    println!("{}", did);
}

#[post("/register")]
pub async fn register(req: HttpRequest) -> HttpResponse {

    println!("register");

    // get DID from request

    // create directory for DID

    HttpResponse::Ok().body(format!("register"))

}

#[post("/login")]
pub async fn login(req: HttpRequest) -> HttpResponse {

    println!("login");

    // The user sends a login request the the server.

    // The server generates a redirect 
    // -> openid://?response_type=id_token&client_id=https%3A%2F%2Frp.example.com%2Fcb&scope=openid%20did_authn&request=<JWT>
    HttpResponse::Ok().body(format!("login"))

    // The redirect is handle by the users user-agent (browser extension, native app, etc.)
    // The user-agent handles DID selection and any user interaction
    // The user sends a response back to the server with a self-issued auth token

}


mod tests {
    use super::*;
    #[test]
    fn it_works() {
        main();
        assert_eq!(2 + 2, 4);
    }
}
