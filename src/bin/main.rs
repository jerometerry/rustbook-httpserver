extern crate httpserver;

use httpserver::options::Options;
use httpserver::webserver::WebServer;

fn main() {
    let options = Options::new(String::from("127.0.0.1:8080"), 4);
    let server = WebServer::new(options);
    server.start();
}
