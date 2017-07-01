extern crate httpserver;

use httpserver::WebServer;

fn main() {
    WebServer::run(String::from("127.0.0.1:8080"), 4);
}
