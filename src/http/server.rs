use std::net::TcpListener;
use crate::CanyonConfig;

/// The Canyon built-in http server.
///
/// Supports up to HTTP 1.1 (2.0?) client-server connections
pub struct HttpServer;

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
        // listener.accept().into_ok().0.

        for stream in listener.incoming() {
            let stream = stream.unwrap(); // TODO Handle client error
            // TODO middleware for track and logging the hosts that make the requests (stream.peer_addr())
            crate::core::net::handle_connection(stream)
            // TODO Impl handle_connection on a trait, to specifically handle every event
        }
    }
}

