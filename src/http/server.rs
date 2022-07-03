use std::net::TcpListener;
use crate::CanyonConfig;

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
        let listener = TcpListener::bind(
            format!("{:?}:{:?}", "127.0.0.1", "7878")
        ).unwrap(); // TODO handle error

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established! Stream: {:?}", &stream);
        }
    }
}

