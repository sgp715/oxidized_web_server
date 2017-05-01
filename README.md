# Oxidized Web Server

For this homework, you will implement a rudimentary web server. The purpose is for you to begin taking advantage of concurrency in Rust.

## The deliverable
The purpose of web_server is to respond to the single command of HTTP 0.9, the GET method, which has the following shape:
    GET /path/to/file HTTP
That is, it is the literal world GET, followed by a blank space, followed by a Unix-style absolute path to a file, followed by another blank space and the literal token HTTP. The following line is a blank line. For forward compatibility, you should also accept newer HTTP versions, which will end their request with a token that includes the version, e.g., HTTP/1.1. And you should skip over any header lines following the request but preceding the blank line.
