use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "hello.html")
        }
        _ =>  ("404 NOT FOUND", "404.html")
    };

    write_file_response(&mut stream, filename, status);
}

fn write_file_response(stream: &mut TcpStream, path: &str, status: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("HTTP/1.1 {status}\r\nContent-Length: {length} \r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}