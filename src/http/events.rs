use std::net::TcpStream;

use crate::core::{net::{NetworkStream}, parsers::Parseable};

use super::methods::HttpMethod;

pub trait Request: NetworkStream {}


/// Represents the structure of some kind of Http TCP request
/// 
/// TODO Docs
pub struct HttpRequest<'a> {
    pub verb: HttpMethod,
    pub uri: &'a str,
    pub http_version: &'a str,
    pub headers: &'a [&'a str],  // TODO replace for dyn allocate type?¿
    pub body: &'a str
}


impl<'a> HttpRequest<'a> {
    pub fn new<T: Request>(request: &mut T) -> Self {
        // Call parse to validate the input data
        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly
        request.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        /// ...
        Self { 
            verb: HttpMethod::GET, 
            uri: "https://somecosa.url", 
            http_version: "HTTP/1.1", 
            headers: &["no-headers-thing"], 
            body: "" 
        }
    }
}