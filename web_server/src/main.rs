/**
 * A web server implemented in Rust
 */

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {

    println!("Binding listener to 127.0.0.1:8080. Press Ctrl+C to quit.");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error processing TcpStream: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {

    let mut buffer = String::new();

    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            println!("{}", buffer);
        },
        Err(_) => {
            println!("Could not read TcpStream");
        },

    }

}
