use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
extern crate regex;
use self::regex::Regex;


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

    let reg = Regex::new(r"HTTP(\\/\d.\d|\b)").unwrap();
    reg.is_match(request)
}

#[test]
fn validate_protocol_format_test(){

    assert_eq!(validate_protocol_format("HTT"), false);
    assert_eq!(validate_protocol_format("HTTp"), false);
    assert_eq!(validate_protocol_format("HTTP/1.1"), true);
    assert_eq!(validate_protocol_format("HTTP/1.0"), true);
    assert_eq!(validate_protocol_format("HTTP/0.9"), true);
    assert_eq!(validate_protocol_format("HTTP/0.8"), false);
    assert_eq!(validate_protocol_format("HTTP"), true);
    assert_eq!(validate_protocol_format("HTTP\r\n"), true);
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

    if validate_request_format(request) == false {
        return (400, "HTTP/1.0 400 Bad Request\r\n<html><body>Very bad...</body></html>".to_owned())
    }

    let dir_path = env::current_dir().unwrap();
    let file = request.split(' ').nth(1).unwrap();
    let filename = dir_path.to_str().unwrap().to_owned() + file;
    if !Path::new(&filename).exists() {
        return (404, "HTTP/1.0 404 Not Found\r\n<html><body>Nope</body></html>".to_owned())
    }

    match File::open(filename) {
        Ok(mut f) => {

            let mut contents = String::new();
            let size = f.read_to_string(&mut contents).unwrap();

            let filetype: &str;
            if file.ends_with(".html") {
                filetype = "html";
            } else {
                filetype = "text";
            }

            let ok_body = format!("HTTP/1.0 200 OK\n\
                                   nsc969-web-server/0.1\n\
                                   Content-type: text/{}\n\
                                   Content-length: {}\n\n\
                                   {}", filetype, size, contents);

            return (200, ok_body);

        },
        Err(_) => {
            return (403, "HTTP/1.0 403 Forbidden\r\n<html><body> Naughty </body></html>".to_owned());
        }
    };

}
