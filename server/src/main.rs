use thread_pool::ThreadPool;

use std::{
    fs,
    io::{Read, Write},
    net::{self, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let threadpool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        threadpool.execute(|| {
            // println!("request: ")
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // println!("{}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let html_file_path = Path::new("assets").join(filename);

    let html_contents = fs::read_to_string(html_file_path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html_contents.len(),
        html_contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
