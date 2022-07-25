use std::{net::{TcpListener, TcpStream}, io::Read};
use super::events::HttpRequest;
use crate::{CanyonConfig, core::net::{NetworkStream, Request}};

/// The Canyon built-in http server.
///
/// Supports up to HTTP 1.1 (2.0?) client-server connections
pub struct HttpServer {

}

impl HttpServer {
    /// Constructor for the [`HttpServer`].
    ///
    /// Sets up a construction around the [`std::net::TcpListener`], which is a *TCP* socket server,
    /// It will bind to a socket address, the one that the user defined in the Canyon config file
    /// and will start listening for incoming *TCP* connections.
    pub fn run(config: CanyonConfig) {
        // TODO Generate the request parser and the response writer as members
        // available to performn some trait defined action


        let listener = TcpListener::bind(
            format!("{}:{}", config.server.ip, config.server.port)
        ).unwrap(); // TODO handle error

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            HttpServer::handle_connection(stream)
        }
    }

    /// Convenient method to read the data coming from a Tcp event
    fn handle_connection(mut stream: impl NetworkStream) {
        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly
        // to the incoming request
    
        stream.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
    
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // The request parse enters in action
        let request: HttpRequest = HttpRequest::new(&buffer[..]);

        // --------------------- RESPONSE EVENTS -----------------------------
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

