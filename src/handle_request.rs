use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
extern crate regex;
use self::regex::Regex;
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

fn generate_ok_body(mut f: File, filename: &str) -> String {

    let mut contents = String::new();
    let size = f.read_to_string(&mut contents).unwrap();

    let filetype: &str;
    if filename.to_owned().ends_with(".html") {
        filetype = "html";
    } else {
        filetype = "text";
    }

    format!("HTTP/1.0 200 OK\n\
            nsc969-web-server/0.1\n\
            Content-type: text/{}\n\
            Content-length: {}\n\n\
            {}", filetype, size, contents)

}

pub fn generate_response(request: &str) -> (i64, String, String) {

    if validate_request_format(request) == false {
        return (400, "HTTP/1.1 400 Bad Request\n\n<html><body>You suck...</body></html>".to_owned(), "[could not parse]".to_owned())
    }

    let dir_path = env::current_dir().unwrap();
    let filename = dir_path.to_str().unwrap().to_owned() + request.split(' ').nth(1).unwrap();
    if Path::new(&filename).exists() {

        let meta = fs::metadata(&filename).unwrap();
        let file_type = meta.file_type();

        let mut file_served = filename.clone();

        if file_type.is_dir() {

            println!("HERE");
            let filename_html = filename.clone() + "index.html";
            let filename_shtml = filename.clone() + "index.shtml";
            let filename_txt = filename.clone() + "index.txt";
            println!("{}", filename_html);
            if Path::new(&filename_html).exists() {
                file_served = filename_html;
            } else if Path::new(&filename_shtml).exists() {
                file_served = filename_shtml;
            } else if Path::new(&filename_txt).exists() {
                file_served = filename_txt;
            } else {
                return (404, "HTTP/1.1 404 Not Found\n\n<html><body> Nope </body></html>".to_owned(), filename);
            }

        }

        match File::open(&file_served) {
            Ok(f) => {

                let ok_body = generate_ok_body(f, &file_served);
                println!("{}", ok_body);

                return (200, ok_body, filename);

            },
            Err(_) => {
                return (403, "HTTP/1.0 403 Forbidden\n\n<html><body> Naughty </body></html>".to_owned(), filename);
            }

        };

    }

    return (404, "HTTP/1.1 404 Not Found\n\n<html><body> Nope </body></html>".to_owned(), filename);

}
