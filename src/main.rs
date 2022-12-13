use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use serde::{Serialize};

#[derive(Serialize, Debug)]
struct Resp {
    current_count: i64,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut response_body = String::from("");
    let mut status_line = "HTTP/1.1 200 OK";
    if request_line == "GET /current HTTP/1.1" {
        response_body = get_current();
    } else if request_line == "POST /increment HTTP/1.1" {
        increment();
    } else if request_line == "POST /decrement HTTP/1.1" {
        decrement();
    } else if request_line == "POST /reset HTTP/1.1" {
        reset()
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
    }


    let length = response_body.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{response_body}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_current() -> String {
    let current = db_interface::read_db().unwrap();
    let resp = Resp{ current_count: current};
    let response = serde_json::to_string(&resp).unwrap();
    response
}

fn increment() {
    db_interface::increment().unwrap()
}

fn decrement() {
    db_interface::decrement().unwrap()
}

fn reset () {
    db_interface::reset().unwrap()
}