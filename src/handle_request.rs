use std::fs::metadata;
use std::fs::File;
use std::path::Path;
use std::env;
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
    assert_eq!(validate_protocol_format("HTTP/0.8"), true);
    assert_eq!(validate_protocol_format("HTTP"), true);

}

fn validate_request_format(request: &str) -> bool {
    let segments: Vec<&str> = request.split(' ').collect();
    if segments.len() == 3{
        if validate_get_format(segments[0])&&validate_protocol_format(segments[2]){
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


pub fn generate_response(request: &str) -> (i64, String, String) {

    if validate_request_format(request) == false {
        return (400, "HTTP/1.1 400 Bad Request\n\n<html><body>You suck...</body></html>".to_owned(), "[could not parse]".to_owned())
    }

    let dir_path = env::current_dir().unwrap();
    let mut filename = dir_path.to_str().unwrap().to_owned() + request.split(' ').nth(1).unwrap();
    if !Path::new(&filename).exists() {

        filename.push_str("/index.html");
        if Path::new(&filename).exists(){
            return (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned(),filename)
        }
        let mut length = filename.len();
        filename.truncate(length-11);
        filename.push_str("/sindex.html");

        if Path::new(&filename).exists(){
            return (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned(),filename)
        }
        let mut length = filename.len();
        filename.truncate(length-12);
        filename.push_str("/index.txt");

        if Path::new(&filename).exists(){
            return (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned(),filename)
        }

        return (404, "HTTP/1.1 404 Not Found\n\n<html><body> Nope </body></html>".to_owned(), filename)
    }

    // let mut file = File::open(filename).unwrap();
    // println!("file: {}", file.is_file());
    // let mut contents = String::new();

    // match file {
    //     Ok(f) => f.read_to_string(&mut contents),
    //     Err(_) => {
    //         println!("Could not find file");
    //     }
    // }

    (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned(),filename)

}
