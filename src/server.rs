use httparse::Request;
use image::io::Reader as ImageReader;
use std::{
    io::{Cursor, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::ascii;

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    // read the request from the stream
    let n = stream.read(&mut buffer)?;
    match n {
        0 => return Ok(()),
        _ => println!("Request received, n: {}", n),
    }

    // parse http request in http request struct
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    let status = req.parse(&buffer).unwrap();

    if status.is_complete() {
        // check that the request method is POST
        if req.method.unwrap() != "POST" {
            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
            let amount = stream.write(response.as_bytes()).unwrap();
            match amount {
                0 => return Ok(()),
                _ => println!("Response sent, n: {}", amount),
            }
            return Ok(());
        }

        // get the content length of the request body
        let content_length = req
            .headers
            .iter()
            .find(|h| h.name == "Content-Length")
            .and_then(|h| std::str::from_utf8(h.value).ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        // read the request body, which is the picture
        let mut picture = vec![0; content_length];
        stream.read_exact(&mut picture).unwrap();

        // do some processing on the picture here...
        let img2 = ImageReader::new(Cursor::new(&picture))
            .with_guessed_format()?
            .decode()?;

        let ascii_img = ascii::to_ascii(img2);

        // send the response
        let status = "HTTP/1.1 200 OK\r\n";
        let content_type = "Content-Type: text/plain\r\n";
        let content_length = format!("Content-Length: {}\r\n", ascii_img.len());

        let response = format!(
            "{}{}{}{}{}",
            status, content_type, content_length, "\r\n", ascii_img
        );

        let written_res = stream.write(response.as_bytes());
        match written_res {
            Ok(n) => println!("Response sent, n: {}", n),
            Err(e) => println!("Error sending response: {}", e),
        }

        stream.flush().unwrap();
        Ok(())
    } else {
        // handle uncomplete request
        Err("Request incomplete".into())
    }
}
