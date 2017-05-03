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

use handle_request::generate_response;

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

                    let (request_type, results,filename) = handle_client(&stream);

                    match results {
                        Some(r) => {

                            let mut file = mutex.lock().unwrap();
                            match *file {
                                Ok(ref mut f) => {
                                    log_request(f, stream.peer_addr().unwrap(),filename, r,request_type);
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

fn handle_client(stream: &TcpStream) -> (i64,Option<(String)>,String) {

    let mut reader = BufReader::new(stream);
    let mut request = String::new();

    let _bytes_read = reader.read_line(&mut request);

    for line in reader.lines() {

        if line.unwrap() == "" {
            break;
        }

    }

    // let remote_addr = stream.peer_addr().unwrap();
    let (request_type, response, filename) = generate_response(&request);

    let writer = BufWriter::new(stream);
    send_response(writer, &response);

    if request_type == 400 {
        return (request_type, None, filename)
    }

    (request_type,Some(request),filename)

}

fn send_response(mut writer: BufWriter<&TcpStream>, response: &str) {

    writer.write(response.as_bytes()).unwrap();

}

fn log_request(mut file: &File, addr: SocketAddr, filename: String, request: String, request_type: i64) {

    let entry = format!("Time: {}, Remote IP: {}, URL:{} Status Code: {}\n",
                        time::now().ctime(), addr,filename, request_type);

    // Uncomment the following line to print log to stdout
    println!("{}", entry);

    let _bytes_written = file.write(entry.as_bytes());
}
