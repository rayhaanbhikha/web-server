use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

use crate::worker::Worker;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

#[derive(Debug)]
pub enum ThreadPoolError {
    InvalidPoolLimit,
}

impl ThreadPool {
    pub fn new(limit: usize) -> Result<Self, ThreadPoolError> {
        if limit == 0 {
            return Err(ThreadPoolError::InvalidPoolLimit);
        }

        let (tx, rx) = channel::<Message>();

        let receiver = Arc::new(Mutex::new(rx));
        let mut workers: Vec<Worker> = Vec::with_capacity(limit);

        for i in 0..limit {
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }

        Ok(Self {
            workers,
            sender: tx,
        })
    }

    pub fn execute<F>(&self, lambda: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(lambda);

        self.sender.send(Message::NewJob(job)).unwrap()
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
            println!("Shutting down worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}
