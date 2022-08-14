use std::collections::HashMap;

use crate::core::net::{NetworkStream, Request};
use super::types::{HttpMethod, Uri, HttpVersion};

/// Represents the structure of an Http request
/// 
/// TODO Docs
#[derive(Debug)]
pub struct HttpRequest {
    pub verb: HttpMethod,
    pub uri: Uri,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl Request for HttpRequest {
    fn is_valid(&self) -> bool {
        todo!()  // TODO Implentation of the valid conditions of a request. 
        // (Find it on a Router based on the path request? if matches a controller, that 
        // condition will be OK, and
        // has valid headers?) has valid http version?
    }
}

impl HttpRequest {
    pub fn new<'a, T: NetworkStream>(stream: &'a mut T) -> Self {
        // Call parse to validate the input data
        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly to the incoming request
        stream.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
        let request_payload = String::from_utf8_lossy(&buffer[..]);  // let binding for longer live the slices

        // Organize the request by first, splitting it by CRLF
        let mut sp = request_payload.split("\r\n");
        // After this, we take the first element on the iterator contains the verb, uri and http version of the request
        let (verb, uri, version) = Self::parse_verb_uri_version(&mut sp);
        // Then, we get (not consume) the last element on the request, that is the body of the Http request
        let body = Self::parse_http_request_body(sp.clone().last().unwrap());  // TODO Alternative to not clone the iterator?
        // Finally, we consume the intermediate elements, the remaining ones in the iterator
        let headers = Self::parse_http_request_headers(&mut sp);

        Self { 
            verb: verb, 
            uri: uri, 
            http_version: version, 
            headers: headers, 
            body: body
        }
    }

    /// Method for parse the first three elements on an Http request
    /// 
    /// It returns a tuple containing the elements parsed and disgregated from the original 
    /// Split<&str> iterator
    fn parse_verb_uri_version<'b>(payload: &'b mut std::str::Split<&str>) -> (HttpMethod, Uri, HttpVersion) {
        let mut method_uri_version = payload
            .next()
            .expect("Something wrong happened getting verb-uri-version")
            .split_ascii_whitespace();

        (
            Self::parse_http_method(method_uri_version.next().unwrap()),
            Self::parse_uri(method_uri_version.next().unwrap()),
            Self::parse_http_version(method_uri_version.next().unwrap())
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

    /// Parses and validates the version of the incoming Http request
    fn parse_http_version<'b>(payload: &'b str) -> HttpVersion {
        match HttpVersion::from_str(payload) {
            Ok(version) => version,
            Err(_) => todo!()
        }
    }

    /// Parses the elements in the iterator that contains the Http request in order to get
    /// the Http headers defined in it, by taking the elements in the sp arg, splitting the
    /// &str remaining there.
    /// 
    /// Every &str it's splitted by (": "), ensuring that the elements are valid ones to have a 
    /// complete definition of an Http header.
    fn parse_http_request_headers<'b>(sp: &'b mut std::str::Split<&str>) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut iter = sp.peekable();

        while iter.peek() != None {
            let next = iter.next();
            if let Some(value_to_parse) = next {
                let parts = value_to_parse.split(": ").collect::<Vec<&str>>();
                let key = parts.get(0);
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

        headers
    }


    /// Parses the Http request body, anullating those ones that are zero-padded ones or discarding the empty ones
    /// TODO Probably the body should be converted to some T type if the headers contains, for example, the application/json?
    /// Some some type should be impl over the response of this method, generificating Self into T,
    /// or better, into custom type, like body, so the return type of the method could be
    /// something like Option<Body<T>>
    fn parse_http_request_body<'b>(body: &'b str) -> Option<String> {
        // First, discard the non desired elemments
        if body.starts_with("\0") || body.is_empty() { return None; }
        Some(body.to_string())
    }
}


/// Unit tests for the methods of the [`HttpRequest`]. Parser methods are the important ones.
/// 
/// TODO Improve test when the result of parsing w'd be a Result<T, E>, representing the possible
/// failure parsing by an incorrect or unsupport value
#[cfg(test)]
mod http_tests {
    use super::*;

    const MOCKED_PAYLOAD: &[&str] = &[
        "GET",
        "http://mocked_uri.net",
        "HTTP/1.1",
        "some_thing_to_mock\r\nheader-thing: some-header-value",
        "\0"
    ];

    #[test]
    fn test_http_method_parser() {
        let http_method = HttpRequest::parse_http_method(&MOCKED_PAYLOAD[0]);
        assert_eq!(http_method, HttpMethod::GET)
    }

    #[test]
    fn test_http_uri_parser() {
        let uri = HttpRequest::parse_uri(&MOCKED_PAYLOAD[1]);
        assert_eq!(uri.uri.as_str(), *&MOCKED_PAYLOAD[1]);  // TODO Still dummy 'cause not fully impl the Uri parser
    }

    #[test]
    fn test_http_version_parser() {
        let http_version = HttpRequest::parse_http_version(&MOCKED_PAYLOAD[2]);
        assert_eq!(http_version, HttpVersion::V1_1)
    }

    #[test]
    fn test_http_headers_parser() {
        let http_headers = HttpRequest::parse_http_request_headers(&mut MOCKED_PAYLOAD[3].split("\r\n"));
        assert!(http_headers.contains_key("header-thing"));
        assert_eq!(http_headers.get("header-thing"), Some(&"some-header-value".to_string()));
    }

    #[test]
    fn test_http_body_parser() {
        let body = HttpRequest::parse_http_request_body(&MOCKED_PAYLOAD[4]);
        let body_2 = HttpRequest::parse_http_request_body("");
        let body_3 = HttpRequest::parse_http_request_body("http-body");
        
        assert_eq!(body, None);
        assert_eq!(body_2, None);
        assert_eq!(body_3, Some("http-body".to_string()));

        assert_ne!(body_3, Some("http-bodybodybody".to_string()));
    }
}