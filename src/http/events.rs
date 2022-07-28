use std::collections::HashMap;

use crate::core::net::{NetworkStream};
use super::types::{HttpMethod, Uri};

pub trait Request {}


/// Represents the structure of some kind of Http TCP request
/// 
/// TODO Docs
#[derive(Debug)]
pub struct HttpRequest {
    pub verb: HttpMethod,
    pub uri: Uri,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Request for HttpRequest {}

impl HttpRequest {
    pub fn new<'a, T: NetworkStream>(stream: &'a mut T) -> Self {
        // Call parse to validate the input data
        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly to the incoming request
        stream.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
        let request_payload = String::from_utf8_lossy(&buffer[..]);  // let binding for longer live the slices

        // Organize the request by first, splitting it by CRLF
        let mut sp = request_payload.split("\r\n");
        println!("Splitted: {:?}", &sp.clone().collect::<Vec<&str>>());

        // After this, we take the first element on the iterator contains the verb, uri and http version of the request
        let (verb, uri, version) = Self::parse_verb_uri_version(&mut sp);
        println!("Method: {:?}, URI: {:?}, Version: {}", &verb, &uri, &version);
        // Then, we get (not consume) the last element on the request, that is the body of the Http request
        let body = sp.clone().last().unwrap();  // TODO Alternative to not clone the iterator?

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut iter = sp.peekable();

        while iter.peek() != None {
            let next = iter.next();
            if let Some(value_to_parse) = next {
                let parts = value_to_parse.split(": ").collect::<Vec<&str>>();
                let key = parts.get(0);
                println!("Key: {:?}", &key);
                if parts.len() == 2 {
                    headers.insert(
                        (*key.expect(&format!("Error getting the header definition: {:?}", &key)))
                            .to_string(), 
                        (*parts.get(1)
                            .expect(&format!("Error getting the header value from: {:?}", &parts)))
                            .to_string()
                    );
                }
            } else { iter.next(); }
        }

        println!("Headers: {:?}", &headers);

        Self { 
            verb: verb, 
            uri: uri, 
            http_version: version, 
            headers: headers, 
            body: body.to_string()
        }
    }

    /// Method for parse the first three elements on an Http request
    /// 
    /// It returns a tuple containing the elements parsed and disgregated from the original 
    /// Split<&str> iterator
    fn parse_verb_uri_version<'b>(payload: &'b mut std::str::Split<&str>) -> (HttpMethod, Uri, String) {
        let mut method_uri_version = payload.next()
            .expect("Something wrong happened getting verb-uri-version")
            .split_ascii_whitespace();

        (
            Self::parse_http_method(method_uri_version.next().unwrap()),
            Self::parse_uri(method_uri_version.next().unwrap()),
            method_uri_version.next().unwrap().to_string()
        )
    }

    /// Parses an http verb from an Http request, returning an [`HttpMethod`] enum type
    /// with a variant representing some valid Http Method
    fn parse_http_method<'b>(payload: &'b str) -> HttpMethod {
        match HttpMethod::from_str(payload) {
            Ok(verb) => verb,
            Err(_) => todo!(),
        }
    }

    /// Parses and validates the URI resource from an Http Request
    fn parse_uri<'b>(payload: &'b str) -> Uri {
        Uri::new(payload)
        // TODO Implement the URI parser
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
        let http_method = HttpRequest::parse_http_method(&MOCKED_PAYLOAD[0]);
        assert_eq!(http_method, HttpMethod::GET)
    }
}