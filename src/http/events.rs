use std::net::TcpStream;

use crate::core::{net::{Request, NetworkStream}, parsers::Parseable};

use super::methods::HttpMethod;

/// Represents the structure of some kind of Http TCP request
/// 
/// TODO Docs
pub struct HttpRequest {
    pub verb: HttpMethod,
    pub uri: &'static str,
    pub http_version: &'static str,
    pub headers: [&'static str; 10],  // TODO replace for dyn allocate type?¿
    pub body: &'static str
}

impl HttpRequest {
    pub fn new(request: &[u8]) -> Self {
        // Call parse to validate the input data

        /// ...
        Self { verb: (), uri: (), http_version: (), headers: (), body: () }
    }
}

impl Request<Self> for HttpRequest {}

impl NetworkStream for HttpRequest {}

impl std::io::Write for HttpRequest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
impl std::io::Read for HttpRequest {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        TcpStream::read(&mut self, buf)
    }
}

impl Parseable for HttpRequest {
    fn parse<'a>(&self, content: &'a str) {
        todo!()
    }
}