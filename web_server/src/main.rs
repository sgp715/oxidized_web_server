/**
 * A web server implemented in Rust
 */

use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {

    println!("Binding listener to 127.0.0.1:8080. Press Ctrl+C to quit.");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error processing TcpStream:");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {

    let mut buffer = String::new();

    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            log_request(&buffer);
        },
        Err(_) => {
            println!("Could not read TcpStream");
        },

    }

}

fn log_request(log: &String) {
    
    // Uncomment the following line to print log to stdout
    println!("{}", log);

    let arc = Arc::new(Mutex::new(OpenOptions::new()
                                            .append(true)
                                            .create(true)
                                            .open("log.txt")));
    let mutex = arc.clone();
    let mut file = mutex.lock().unwrap();

    match *file {
        Ok(ref mut f) => {
            let _bytes_written = f.write(log.as_bytes());
        },
        Err(_) => {
            println!("Error writing to log file");
        }
        
    }
}
