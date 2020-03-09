use rspotify::{oauth2::SpotifyOAuth, util::request_token};
use std::{
  io::prelude::*,
  net::{TcpListener, TcpStream},
};

pub fn redirect_uri_web_server(spotify_oauth: &mut SpotifyOAuth, port: u16) -> Result<String, ()> {
  let listener = TcpListener::bind(format!("127.0.0.1:{}", port));

  match listener {
    Ok(listener) => {
      request_token(spotify_oauth);

      for stream in listener.incoming() {
        match stream {
          Ok(stream) => {
            if let Some(url) = handle_connection(stream) {
              return Ok(url);
            }
          }
          Err(e) => {
            println!("Error: {}", e);
          }
        };
      }
    }
    Err(e) => {
      println!("Error: {}", e);
    }
  }

  Err(())
}

fn handle_connection(mut stream: TcpStream) -> Option<String> {
  // The request will be quite large (> 512) so just assign plenty just in case
  let mut buffer = [0; 1000];
  let _ = stream.read(&mut buffer).unwrap();

  // convert buffer into string and 'parse' the URL
  match String::from_utf8(buffer.to_vec()) {
    Ok(request) => {
      let split: Vec<&str> = request.split_whitespace().collect();

      if split.len() > 1 {
        respond_with_success(stream);
        return Some(split[1].to_string());
      }

      respond_with_error("Malformed request".to_string(), stream);
    }
    Err(e) => {
      respond_with_error(format!("Invalid UTF-8 sequence: {}", e), stream);
    }
  };

  None
}

fn respond_with_success(mut stream: TcpStream) {
  let contents = include_str!("redirect_uri.html");

  let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn respond_with_error(error_message: String, mut stream: TcpStream) {
  println!("Error: {}", error_message);
  let response = format!(
    "HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request - {}",
    error_message
  );

  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
