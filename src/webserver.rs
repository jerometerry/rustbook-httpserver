use std::net::TcpListener;

use options::Options;
use threadpool::ThreadPool;
use handler::ConnectionHandler;

pub struct WebServer;

impl WebServer {
    pub fn run(options: Options) {
        let listener = TcpListener::bind(&options.addr).unwrap();
        let pool = ThreadPool::new(options.workers);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| { ConnectionHandler::handle(stream); });
        }
    }
}
