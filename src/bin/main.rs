extern crate httpserver;

use httpserver::WebServer;

fn main() {
    let server = WebServer::new(String::from("127.0.0.1:8080"), 4);
    server.run();
}
