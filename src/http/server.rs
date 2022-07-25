use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use super::events::{HttpRequest, Request};
use crate::{CanyonConfig, core::{net::NetworkStream, parsers::Parseable}};

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
        ).unwrap(); // TODO handle binding error

        for stream in listener.incoming() {
            let stream = stream.unwrap(); // TODO Handle client error
            HttpServer::handle_connection(stream)
        }
    }

    /// Convenient method to read the data coming from a Tcp event
    /// 
    /// TODO Generify this concept into an Struct, that it's associated fn
    /// `handle_connection` receives objects that implements NetworkStream
    fn handle_connection(mut stream: impl NetworkStream) {
        let http_req = HttpRequest::new(&mut stream);
        // --------------------- RESPONSE EVENTS -----------------------------
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        // stream.write(response.as_bytes()).unwrap();
        // stream.flush().unwrap();
    }
}

