fn validate_get_format(request: &str) -> bool {
    false
}

#[test]
fn validate_get_format_test(){
    assert_eq!(validate_get_format("GEt /main.rs HTTP"), false);
    assert_eq!(validate_get_format("GE /main.rs HTTP"), false);
    assert_eq!(validate_get_format("GT /main.rs HTTP"), false);
    assert_eq!(validate_get_format("GET /main.rs HTTP"), true);
}

fn validate_file_format(request: &str) -> bool {
    false
}

#[test]
fn validate_file_format_test(){

    assert_eq!(validate_file_format("GET main.rs HTTP"), false);
    assert_eq!(validate_file_format("GET /main/ HTTP"), false);
    assert_eq!(validate_file_format("GET /main.rs/ HTTP"), false);
    assert_eq!(validate_file_format("GET /main.rs HTTP"), true);

}

fn validate_protocol_format(request: &str) -> bool {
    false
}

#[test]
fn validate_protocol_format_test(){

    assert_eq!(validate_protocol_format("GET /main.rs HTTP"), false);
    assert_eq!(validate_protocol_format("GET /main.rs HTT"), false);
    assert_eq!(validate_protocol_format("GET /main.rs HTTp"), false);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP/"), false);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP/1.1"), true);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP/1.0"), true);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP/0.9"), true);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP/0.8"), false);
    assert_eq!(validate_protocol_format("GET /main.rs HTTP"), true);

}

fn validate_request_format(request: &str) -> bool {
    false
}

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
        return (400, "HTTP/1.1 400 Bad Request\n\n<html><body>You suck...</body></html>".to_owned())
    }

    (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned())

}
