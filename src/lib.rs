use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct WebServer;

impl WebServer {
    pub fn run(addr: String, max_workers: usize) {
        let listener = TcpListener::bind(&addr).unwrap();
        let pool = ThreadPool::new(max_workers);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| { WebServer::handle_connection(stream); });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let data = WebServer::read(&mut stream);
        let (status, filename) = WebServer::get_response(data);
        let html = WebServer::get_file_contents(filename);
        WebServer::write_response(stream, status, &html);
    }

    fn get_response(buffer: [u8; 512]) -> (&'static str, &'static str) {
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        if buffer.starts_with(get) {
            ("200 OK", "hello.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "hello.html")
        } else {
            ("404 NOT FOUND", "404.html")
        }
    }

    fn read(mut stream: &TcpStream) -> [u8; 512] {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        buffer
    }

    fn write_response(mut stream: TcpStream, status: &str, contents: &str) {
        let response = WebServer::format_response(status, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn format_response(status: &str, contents: &str) -> String {
        let http_version = "HTTP/1.1";
        format!("{} {}\r\n\r\n{}", http_version, status, contents)
    }

    fn get_file_contents(filename: &str) -> String {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.execute();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

trait JobExecutor {
    fn execute(self: Box<Self>);
}

impl<F: FnOnce()> JobExecutor for F {
    fn execute(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<JobExecutor + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}
