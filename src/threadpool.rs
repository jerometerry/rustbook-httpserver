use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use worker::Worker;
use message::Message;

pub struct ThreadPool {
    workers: Vec<Worker>,
    work_dispatcher: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            work_dispatcher: sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        self.dispatch_job(f);
    }

    fn dispatch_job<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let msg = Message::RunJob(Box::new(f));
        self.work_dispatcher.send(msg).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.work_dispatcher.send(Message::Terminate).unwrap();
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
