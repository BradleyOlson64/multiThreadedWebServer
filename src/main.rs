use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
fn main() {
    let outcome = open_connections().err();
    match outcome {
        Some(err) => println!("{:?}", err),
        None => ()
    }
}

fn open_connections() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
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

    println!("Request: {:#?}", http_request);
    Ok(())
}