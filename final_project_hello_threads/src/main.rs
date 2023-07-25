use final_project_hello_threads::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("disconnect");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap(); //? 為了把Result<TcpStream, Error>轉換成TcpStream
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
    println!("--------------------------------------------------");

    //?----------------------------------------------------------------
    //? 上面為請求 request(GET)
    //? Method Request-URI HTTP-Version CRLF
    //? headers CRLF
    //? message-body

    //? 接著寫 response
    //? HTTP-Version Status-Code Reason-Phrase CRLF
    //? headers CRLF
    //? message-body

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // {
    // if buffer.starts_with(get) {
    //     let content = fs::read_to_string("hello.html").unwrap();
    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-len: {}\r\n\r\n{}",
    //         content.len(),
    //         content
    //     );
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     let error_message = fs::read_to_string("404.html").unwrap();
    //     let err_response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", error_message);
    //     stream.write(err_response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
    // }
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
