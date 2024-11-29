pub use std::thread;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    time::Duration,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // HTTP Request
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

//     Request: [
    // Method Request-URI HTTP-Version CRLF
        //     "GET / HTTP/1.1",      
    // headers CRLF
        //     "Host: 127.0.0.1:7878",
        //     "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
        //     "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        //     "Accept-Language: en-US,en;q=0.5",
        //     "Accept-Encoding: gzip, deflate, br",
        //     "DNT: 1",
        //     "Connection: keep-alive",
        //     "Upgrade-Insecure-Requests: 1",
        //     "Sec-Fetch-Dest: document",
        //     "Sec-Fetch-Mode: navigate",
        //     "Sec-Fetch-Site: none",
        //     "Sec-Fetch-User: ?1",
        //     "Cache-Control: max-age=0",
    // message-body
        // No body for GET
// ]

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {http_request:#?}");
    // let response = "HTTP/1.1 200 OK\r\n\r\nWelcome";

    let status_line = buf_reader.lines().next().unwrap().unwrap();
    println!("Request...........: {status_line}");

    let status_line_vec = status_line.split(" ").collect::<Vec<&str>>();
    let (method, uri) = (status_line_vec[0], status_line_vec[1]);

    // println!("Request: {method}\nHTTP Method: {uri}\nRequest: {req_line}");


    // if method == "GET" && uri == "/" { 
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();
    //     let response =
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //         stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // let (status_line, filename) = if method == "GET" && uri == "/" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    // let res = "./public".to_owned()+uri;
    // let res = res.as_str();
    let (status_line, filename) = match (method,uri) {
        ("GET","/") => ("HTTP/1.1 200 OK", "hello.html"),
        ("GET","/sleep") => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 202 DELAYED", "hello.html")
        },
        // ("GET",_) => {

        //     ("HTTP/1.1 200 OK", res)
        // },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // HTTP Response
    // HTTP-Version Status-Code Reason-Phrase CRLF
        // Reason-Phrase is "OK"
    // headers CRLF
    // message-body
}
