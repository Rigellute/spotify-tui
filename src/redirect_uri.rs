use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, TryRecvError};

pub fn redirect_uri_web_server(thread_reciever: Receiver<()>) {
    let listener = TcpListener::bind("127.0.0.1:8888");

    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_connection(stream);

                        // Listen for any signal to break out of this loop and close the server
                        match thread_reciever.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => {
                                break;
                            }
                            Err(TryRecvError::Empty) => {}
                        }
                    }
                    Err(e) => {
                        println!("Error running redirect uri webserver {}", e);
                    }
                };
            }
        }
        Err(e) => {
            println!("Error running redirect uri webserver {}", e);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // The request will be quite large (> 512) so just assign plenty just in case
    let mut buffer = [0; 1000];
    stream.read_exact(&mut buffer).unwrap();

    let contents = fs::read_to_string("redirect_uri.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
