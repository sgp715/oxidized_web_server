/**
 * A web server implemented in Rust
 */

use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

extern crate time;

use handle_request::parse_request;

mod handle_request;

fn main() {

    println!("Binding listener to 127.0.0.1:8080. Press Ctrl+C to quit.");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let arc = Arc::new(Mutex::new(OpenOptions::new()
                                            .append(true)
                                            .create(true)
                                            .open("log.txt")));

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mutex = arc.clone();
                thread::spawn(move || {

                    let results = handle_client(stream);

                    match results {
                        
                        Some(r) => {

                            let (addr, request, response) = r;

                            // figurue out how to send response
                            // some concurrent stuff

                            let mut file = mutex.lock().unwrap();
                            match *file {
                                Ok(ref mut f) => {
                                    log_request(f, addr, response);
                                },
                                Err(_) => {
                                    println!("Error writing to file");
                                }
                            }
                        },
                        _ => print!("Bad request")
                    };
                });
            }
            Err(_) => {
                println!("Error processing TcpStream:");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Option<(SocketAddr, String, String)> {

    let mut buffer = String::new();

    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            println!("Handling client");
        },
        Err(_) => {
            println!("Could not read TcpStream");
        },
    }

    let (request_type, request) = parse_request(buffer);

    match request_type {
        400 =>  { return None },
        _ =>    { println!("Valid request!") }
    }

    let remote_addr = stream.peer_addr().unwrap();

    Some((remote_addr, "[request]".to_owned(), "[response]".to_owned()))

}

fn log_request(mut file: &File, addr: SocketAddr, request: String) {

    let entry = format!("Time: {}, Remote IP: {}, Request: {}\n",
                        time::now().ctime(), addr, request);

    // Uncomment the following line to print log to stdout
    //println!("{}", entry);

    let _bytes_written = file.write(entry.as_bytes());
}
