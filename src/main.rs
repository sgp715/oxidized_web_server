/**
 * A web server implemented in Rust
 */

use std::thread;
use std::io::{BufReader, BufWriter};
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

                            let (addr, request) = r;

                            let mut file = mutex.lock().unwrap();
                            match *file {
                                Ok(ref mut f) => {
                                    log_request(f, addr, request);
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

fn handle_client(mut stream: TcpStream) -> Option<(SocketAddr, String)> {

    let mut reader = BufReader::new(&stream);
    let mut request = String::new();

    reader.read_line(&mut request);

    for line in reader.lines() {
        if line.unwrap() == "" {
            break;
        }
    }

    let remote_addr = stream.peer_addr().unwrap();
    let request_type = parse_request(request);

    let mut writer = BufWriter::new(&stream);
    match request_type {
        400 =>    {
            send_response(writer, "HTTP/1.1 400 Bad Request\n\n<html><body>You suck...</body></html>");
            return None
        },
        _ => {
            send_response(writer, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>");
        }
    }

    Some((remote_addr, "[request]".to_owned()))

}

fn send_response(mut writer: BufWriter<&TcpStream>, response: &str) {

    writer.write(response.as_bytes()).unwrap();

}

fn log_request(mut file: &File, addr: SocketAddr, request: String) {

    let entry = format!("Time: {}, Remote IP: {}, Request: {}\n",
                        time::now().ctime(), addr, request);

    // Uncomment the following line to print log to stdout
    //println!("{}", entry);

    let _bytes_written = file.write(entry.as_bytes());
}
