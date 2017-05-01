/**
 * A web server implemented in Rust
 */

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::File;

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
    //println!("{}", log);

    let file = File::create("log.txt");
    match file {
        Ok(mut file) => {
            let _bytes_written = file.write(log.as_bytes());
        },
        Err(_) => {
            println!("Error creating log file");
        }
        
    }
}
