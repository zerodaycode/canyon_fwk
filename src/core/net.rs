use std::{net::TcpStream, time::Instant};

use crate::http::events::HttpRequest;

/// Defines a type that represents some network connection
pub trait NetworkStream: std::io::Read + std::io::Write {}

/// New type pattern for declare generic [`NetworkStream`] events
/// of [`std::net::TcpStream`] type
impl NetworkStream for TcpStream {}

/// Represents a generification over a net request event.
pub trait Request {
    /// The state after parsing a Networking request
    fn is_valid(&self) -> bool;
}


/// Convenient method to read the data coming from a Network event
/// 
/// The idea it's that, in the future, this will be the entry point for handle
/// Http connections, udp connections...
pub fn handle_connection(mut stream: impl NetworkStream) {
    let start = Instant::now();
    let http_req = HttpRequest::new(&mut stream); // TODO impl parse instead of new
    println!("Http request: {:?}\nElapsed: {:?}", &http_req, &start.elapsed()); // TODO impl measurement crate?
    // --------------------- RESPONSE EVENTS -----------------------------
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


/// Some networking event that could be read in many ways and has a custom
/// parser that it's able to understand and translate the request to some
/// implementor type
// pub trait NetReadable<T>: Parseable {}
// impl Parseable for TcpStream {
//     fn parse<'a>(&self, content: &'a str) {
//         todo!()
//     }
// }
// impl<T> NetReadable<T> for TcpStream {}


/// TODO Docs
/// 
/// T where represents some kind of custom user-defined type for an net io
/// response (should be serializable, depending on the response type)
pub struct _Response<T: Request> {
    pub request: T
}

impl<T: Request> _Response<T> {
    /// Returns an Http response for a valid Http request
    pub fn _from_request<'a, R>(_request: &'a dyn Request) -> R {
        todo!()
    }
}