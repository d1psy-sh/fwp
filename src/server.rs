use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut req = vec![];
    let n = stream.read_to_end(&mut req);
    match n {
        Ok(_) => {
            println!("{} bytes read", n.unwrap());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("{:?}", req);
}
