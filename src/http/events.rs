use std::{net::TcpStream, borrow::Cow};

use crate::core::{net::{NetworkStream}, parsers::Parseable};

use super::types::HttpMethod;

pub trait Request {}


/// Represents the structure of some kind of Http TCP request
/// 
/// TODO Docs
#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub verb: HttpMethod,
    pub uri: &'a str,
    pub http_version: &'a str,
    pub headers: &'a [&'a str],  // TODO replace for dyn allocate type?¿
    pub body: &'a str
}

impl Request for HttpRequest<'_> {}

impl<'a> HttpRequest<'a> {
    pub fn new<T: NetworkStream>(stream: &'a mut T) -> Self {
        // Call parse to validate the input data
        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly
        // to the incoming request
        stream.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
        let request_payload = String::from_utf8_lossy(&buffer[..]);  // let binding for longer live the slices
        // print!("Request payload to parse: {:?}", request_payload);

        // Organize the request by first, splitting it by CRLF
        let mut sp = request_payload.split("\r\n");
        // After this, the first element on the iterator contains the verb, uri and http version of the request
        let method_uri_version = sp.next().unwrap_or_default().split_ascii_whitespace()
            .collect::<Vec<&str>>();
        println!("Method uri version: {:?}", &method_uri_version);

        let body = &sp.last().unwrap();
        print!("Body: {:?}", &body);

        let headers = &sp.collect::<Vec<&str>>();

        // println!("Request payload consumed Headers: {:?}", &);
        /// ...
        Self { 
            verb: Self::parse_http_method(&method_uri_version), 
            uri: "https://somecosa.url", 
            http_version: "HTTP/1.1", 
            headers: &["no-headers-thing"], 
            body: "" 
        }
    }

    /// Parses http verbs TODO custom http response code if parsing fails
    fn parse_http_method(payload: &[&str]) -> HttpMethod {
        let verb = payload.get(0);
        match verb {
            Some(v) => match HttpMethod::from_str(*v) {
                Ok(verb) => verb,
                Err(_) => todo!(),
            },
            None => todo!(),
        }
    }

    /// URI parser
    fn parse_uri(payload: &Vec<String>) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCKED_PAYLOAD: &[&str] = &[
        "GET",
        "http://mocked_uri.net",
        "HTTP/1.1",
        "no-headers-thing",
        ""
    ];

    #[test]
    fn test_http_method_parser() {
        let http_method = HttpRequest::parse_http_method(&MOCKED_PAYLOAD);
        assert_eq!(http_method, HttpMethod::GET)
    }
}