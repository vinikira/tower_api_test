#[macro_use]
extern crate tower_web;
extern crate tokio;
#[macro_use]
extern crate serde_json;

use std::net::SocketAddr;
use serde_json::Value;
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct Heores;

impl_web! {
    impl Heores {
        #[get("/")]
        #[content_type("application/json")]
        fn index(&self) -> Result<Value, ()> {
            Ok(json!(["value1", "value2"]))
        }
    }
}

fn main() {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("wrong ip.");

    println!("Listening on http://{}", addr);

    ServiceBuilder::new().resource(Heores).run(&addr).unwrap();
}
