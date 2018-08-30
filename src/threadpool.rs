use std::sync::mpsc; //creates a channel
use std::thread;

use std::sync::Arc; //type will let multiple workers own the receiver
use std::sync::Mutex; //will ensure that only one worker gets a job from the receiver at a time

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

//struct Job;
type Job = Box<FnOnce() + Send + 'static>; //type alias

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); //create channel
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

           for id in 0..size {
               // create some threads and store them in the vector

               //workers.push(Worker::new(id, receiver)); //cannot be distributed among workers
               //solution
               workers.push(Worker::new(id, Arc::clone(&receiver)));
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
           self.sender.send(job).unwrap(); // send job down the sending end of the channel 
       }
}

struct Worker {
       id: usize,
       thread: thread::JoinHandle<()>,
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
     let thread = thread::spawn(|| {
         receiver;
     });

       Worker {
        id,
        thread,
       }
   }
}
