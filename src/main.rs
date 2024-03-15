use multi_threaded_web_server::thread_pool::ThreadPool;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const HTTP_VERSION: &str = "HTTP/1.1";
fn main() {
    //TODO: Should create custom error type that encompasses all
    //      errors in this program. Map other error types into the
    //      general error type.
    match open_connections().err() {
        Some(err) => println!("{:?}", err),
        None => (),
    }
}

fn open_connections() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err))?;
    let pool = ThreadPool::build(2)?;

    for stream in listener.incoming() {
        let stream = stream?;
        pool.execute(|| -> std::io::Result<()> {
            handle_connection(stream)?;
            Ok(())
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let maybe_lines = buf_reader.lines();
    let mut http_request: Vec<_> = vec![];
    for maybe_line in maybe_lines {
        let line = maybe_line?;
        if !line.is_empty() {
            http_request.push(line);
        } else {
            break;
        }
    }
    let request_line = http_request[0].clone();
    let request_enum = parse_request(&request_line);

    match request_enum {
        RequestType::Hello => send_html_response(&mut stream, "HTTP/1.1 200 OK", "hello.html")?,
        RequestType::Sleep(duration) => {
            thread::sleep(duration);
            send_html_response(&mut stream, "HTTP/1.1 200 OK", "hello.html")?
        }
        RequestType::NotFound => {
            send_html_response(&mut stream, "HTTP/1.1 404 NOT FOUND", "404.html")?
        }
        _ => (),
    }

    //println!("Request: {:#?}", http_request);
    Ok(())
}

fn send_html_response(
    stream: &mut TcpStream,
    status_line: &str,
    response_file: &str,
) -> std::io::Result<()> {
    let read_result = fs::read_to_string(response_file);
    let contents = match read_result {
        Ok(result) => result,
        Err(error) => format!("{error}").to_string(),
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn parse_request(request_string: &str) -> RequestType {
    let parts: Vec<&str> = request_string.split_whitespace().collect();
    if parts.len() != 3 {
        return RequestType::InvalidFormat;
    }
    if parts[0] != "GET" {
        return RequestType::InvalidNotGet;
    }
    if parts[2] != HTTP_VERSION {
        return RequestType::WrongHTMLVersion;
    }
    match parts[1] {
        "/" => RequestType::Hello,
        "/sleep" => RequestType::Sleep(Duration::from_secs(5)),
        _ => RequestType::NotFound,
    }
}

#[derive(Debug)]
enum RequestType {
    Hello,
    Sleep(Duration),
    NotFound,
    InvalidFormat,
    InvalidNotGet,
    WrongHTMLVersion,
}
