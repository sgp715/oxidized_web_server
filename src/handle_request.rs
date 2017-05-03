use std::fs::metadata;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;

fn validate_get_format(request: &str) -> bool {
    if request == "GET"{
        return true;
    }
    return false;
}

#[test]
fn validate_get_format_test(){
    assert_eq!(validate_get_format("GEt"), false);
    assert_eq!(validate_get_format("GE"), false);
    assert_eq!(validate_get_format("GT"), false);
    assert_eq!(validate_get_format("GET"), true);
}


fn validate_file_format(request: &str) -> bool {
    let mut chars = request.chars();
    if chars.next() == Some('/') {
        if chars.last() == Some('/'){
            return false;
        }
        else{
            return true;
        }
    }
    return false;
}

#[test]
fn validate_file_format_test(){

    assert_eq!(validate_file_format("main.rs"), false);
    assert_eq!(validate_file_format("/main/"), false);
    assert_eq!(validate_file_format("/main.rs/"), false);
    assert_eq!(validate_file_format("/main.rs"), true);

}

fn validate_protocol_format(request: &str) -> bool {
    let mut chars = request.chars().peekable();
    if chars.next() ==Some('H')&&chars.next()==Some('T')&&chars.next()==Some('T')&&chars.next()==Some('P'){
        if chars.peek()==None{
            return true;
        }
        if chars.next() == Some('/'){
            if chars.peek() ==Some(&'0'){
                chars.next();
                if chars.next() ==Some('.')&&chars.next()==Some('9'){
                    return true;
                }
            }
            else if chars.peek() == Some(&'1'){
                chars.next();
                if chars.next() ==Some('.')&&(chars.peek()==Some(&'0')||chars.peek()==Some(&'1')){
                    return true;
                }
            }
        }
    }
    return false;
}

#[test]
fn validate_protocol_format_test(){

    assert_eq!(validate_protocol_format("HTT"), false);
    assert_eq!(validate_protocol_format("HTTp"), false);
    assert_eq!(validate_protocol_format("HTTP/"), false);
    assert_eq!(validate_protocol_format("HTTP/1.1"), true);
    assert_eq!(validate_protocol_format("HTTP/1.0"), true);
    assert_eq!(validate_protocol_format("HTTP/0.9"), true);
    assert_eq!(validate_protocol_format("HTTP/0.8"), false);
    assert_eq!(validate_protocol_format("HTTP"), true);

}

fn validate_request_format(request: &str) -> bool {
    let segments: Vec<&str> = request.split(' ').collect();
    if segments.len() == 3{
        if validate_get_format(segments[0])&&validate_file_format(segments[1])&&validate_protocol_format(segments[2]){
            return true;
        }
    }
    return false;
}

#[test]
fn validate_request_format_test(){

    assert_eq!(validate_request_format(" GET /main.rs HTTP"), false);
    assert_eq!(validate_request_format("GET  /main.rs HTTP"), false);
    assert_eq!(validate_request_format("GET /main.rs  HTTP"), false);
    assert_eq!(validate_request_format("GET /main.rs HTTP "), false);
    assert_eq!(validate_request_format(" ueau GET /main.rs HTTP"), false);
    assert_eq!(validate_request_format("GET /main.rs HTTP aueau"), false);
    assert_eq!(validate_request_format("GET /main.rs aueau HTTP"), false);
    assert_eq!(validate_request_format("GET /main.rs HTTP"), true);

}


pub fn generate_response(request: &str) -> (i64, String) {

    // if validate_request_format(request) == false {
    //     return (400, "HTTP/1.0 400 Bad Request\n\n<html><body>You suck...</body></html>".to_owned())
    // }

    let dir_path = env::current_dir().unwrap();
    let filename = dir_path.to_str().unwrap().to_owned() + request.split(' ').nth(1).unwrap();
    if !Path::new(&filename).exists() {
        return (404, "HTTP/1.0 404 Not Found\n\n<html><body> Nope </body></html>".to_owned())
    }

    match File::open(filename) {
        Ok(mut f) => {

            let mut contents = String::new();
            let size = f.read_to_string(&mut contents).unwrap();

            let mut text = "plain";
            // if &filename[(&filename.len() - 5)..] == ".html" {
            //     text = "html";
            // }

            let mut ok_body = format!("HTTP/1.0 200 OK\n \
                                      Content-type: text/{}\n \
                                      Content-length: {}\n \
                                      server-name\n \
                                      {}", text, size, contents);

            return (200, ok_body);

        },
        Err(_) => {
            return (403, "HTTP/1.0 403 Forbidden\n\n<html><body> Naughty </body></html>".to_owned());
        }
    };

}
