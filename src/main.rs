#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::thread::JoinHandle;

fn handle_incoming(stream: &mut TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read != 0 {
            stream.write_all(b"+PONG\r\n").unwrap();
        } else {
            break;
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut handles: Vec<_> = vec![]; 

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let handle = thread::spawn(move || {
                    handle_incoming(&mut _stream);
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
