pub fn generate_response(request: &str) -> (i64, String) {


    (200, "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>".to_owned())

}


fn validate_get_format() -> bool {
    false
}

#[test]
fn validate_get_format_test(){
    assert_eq!(generate_response("GEt /main.rs HTTP"), false);
    assert_eq!(generate_response("GE /main.rs HTTP"), false);
    assert_eq!(generate_response("GT /main.rs HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTTP"), true);
}

fn validate_file_format() -> bool {
    false
}

#[test]
fn validate_get_file_test(){

    assert_eq!(generate_response("GET main.rs HTTP"), false);
    assert_eq!(generate_response("GET /main/ HTTP"), false);
    assert_eq!(generate_response("GET /main.rs/ HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTTP"), true);

}

fn validate_protocol_format() -> bool {
    false
}

#[test]
fn validate_protocol_format_test(){

    assert_eq!(generate_response("GET /main.rs HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTT"), false);
    assert_eq!(generate_response("GET /main.rs HTTp"), false);
    assert_eq!(generate_response("GET /main.rs HTTP/"), false);
    assert_eq!(generate_response("GET /main.rs HTTP/1.1"), true);
    assert_eq!(generate_response("GET /main.rs HTTP/1.0"), true);
    assert_eq!(generate_response("GET /main.rs HTTP/0.9"), true);
    assert_eq!(generate_response("GET /main.rs HTTP/0.8"), false);
    assert_eq!(generate_response("GET /main.rs HTTP"), true);

}

fn validate_http_format() -> bool {
    false
}

fn validate_http_test(){

    assert_eq!(generate_response(" GET /main.rs HTTP"), false);
    assert_eq!(generate_response("GET  /main.rs HTTP"), false);
    assert_eq!(generate_response("GET /main.rs  HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTTP "), false);
    assert_eq!(generate_response(" ueau GET /main.rs HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTTP aueau"), false);
    assert_eq!(generate_response("GET /main.rs aueau HTTP"), false);
    assert_eq!(generate_response("GET /main.rs HTTP"), true);

}
