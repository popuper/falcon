use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::pool::Message::{NewJob, Terminate};

/*
this content is quote The Book last chapter: the final project
@see https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
*/
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

// use as thread
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, rcv: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            println!("Worker {} got a job; executing.", id);
            match rcv.lock().unwrap().recv().unwrap() {
                NewJob(j) => j(),
                Terminate => break,
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct Pool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl Pool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, rec) = mpsc::channel();
        let rec = Arc::new(Mutex::new(rec));
        for id in 0..size {
            workers.push(Worker::new(id, rec.clone()));
        }
        Self {
            workers,
            sender,
        }
    }
    // Send NewJob to Worker let it running
    pub fn run<F: FnOnce() + Send + 'static>(&self, func: F) {
        self.sender.send(NewJob(Box::new(func))).unwrap();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("shut down worker{}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

