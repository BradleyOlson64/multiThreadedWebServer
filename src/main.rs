use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs;
fn main() {
    match open_connections().err() {
        Some(err) => println!("{:?}", err),
        None => ()
    }
}

fn open_connections() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err))?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
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

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html")?;
        let len = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes())?;
    } else {
        //Do nothing
    }
    

    println!("Request: {:#?}", http_request);
    Ok(())
}