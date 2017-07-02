extern crate httpserver;

use httpserver::Options;
use httpserver::WebServer;

fn main() {
    let options = Options::new(String::from("127.0.0.1:8080"), 4);
    WebServer::run(options);
}
