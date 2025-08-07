#![allow(unused_imports)]

mod parser;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader};
use std::thread;
use std::thread::JoinHandle;
use parser::BasicRedisParser;

use crate::parser::RedisParser;

fn handle_incoming(stream: TcpStream) {
    let parser = BasicRedisParser::new();
    let mut reader = BufReader::new(stream);
    let results = parser.parse_stream(&mut reader);

    // stream.write_all(b"+PONG\r\n").unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut handles: Vec<_> = vec![]; 

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(move || {
                    handle_incoming(stream);
                });
                handles.push(handle);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
