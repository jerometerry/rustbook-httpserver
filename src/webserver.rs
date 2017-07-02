use std::net::TcpListener;

use options::Options;
use threadpool::ThreadPool;
use handler::ConnectionHandler;

pub struct WebServer {
    addr: String,
    pool: ThreadPool,
}

impl WebServer {
    pub fn new(options: Options) -> WebServer {
        WebServer {
            addr: options.addr,
            pool: ThreadPool::new(options.workers)
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        for connection in listener.incoming() {
            let connection = connection.unwrap();
            self.process(|| ConnectionHandler::handle(connection));
        }
    }

    fn process<F>(&self, handler: F)
        where F: FnOnce() + Send + 'static {
        self.pool.execute(handler);
    }
}
