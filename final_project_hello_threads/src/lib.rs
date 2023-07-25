use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// struct RequestJob;
//? 使用類型別名(type alias)也就是『綽號』。
type RequestJobClosure = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(RequestJobClosure),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Workers>, //? 包裝成一個workers，這是一個數據結構
    sender: mpsc::Sender<Message>,
    //? 也就是說ThreadPool他是一個結構，內部資料是裝著很多“工人(Workers)”這個struct的Vec
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        //? 這個函數是創建指定數量的worker形成一個線程池
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Workers::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); //? Box裡面裝的是閉包
        self.sender.send(Message::NewJob(job)).unwrap();
        //? 這邊的 self.sender就是 ThreadPool裡面的sender
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        } //? 把他移出來，本身就為None
    }
}

struct Workers {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
//? 這是創立一個工人的數據結構，這樣可以再附加編號，以及進一步封裝。

impl Workers {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Workers {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            //? 這邊是接收方，也就是當我建立時先待命，
            //? 當channel開通把閉包send進來receiver端，
            //? 給他解讀。
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job, executing...", id);
                    job();
                }
                Message::Terminate => {
                    println!("Shutting down");
                    break;
                }
            }
        });

        Workers {
            id,
            thread: Some(thread),
        }
    }
}
