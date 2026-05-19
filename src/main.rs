use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
fn new(size :usize) -> ThreadPool{
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec :: with_capacity(size);
    for id in 0..size {
    workers.push(Worker::new(id, Arc::clone(&receiver)));
    
        }
ThreadPool { workers, sender }
    }
        fn execute<F> (&self,f:F)
        where
        F:FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }


}
impl Worker {
    fn new (id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let start = std::time::Instant::now();
let buf_reader = BufReader::new(&stream);
let request_line = buf_reader
.lines().next()
.unwrap().unwrap();
   
 let path = request_line.split_whitespace().nth(1).unwrap();

    let file_path = match request_line.as_str() {
    "GET / HTTP/1.1" => "public/index.html",
    "GET /about HTTP/1.1" => "public/about.html",
    "GET /about.html HTTP/1.1" => "public/about.html",
    _ => "public/404.html",
};

        let (status, body) = match fs::read_to_string(&file_path) {
            Ok(contents) => ("HTTP/1.1 200 OK", contents),
            Err(_) => ("HTTP/1.1 404 NOT FOUND", "<h1>404 - Page Not Found</h1>".to_string()),
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            status,
            body.len(),
            body
        );
stream.write_all(response.as_bytes()).unwrap();
println!("[{}] {} {}ms", request_line, status, start.elapsed().as_millis());
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on http://127.0.0.1:8080");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(move || { 
            handle_connection(stream);
        });

       
    }
    
}
