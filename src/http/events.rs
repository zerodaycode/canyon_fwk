use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::core::net::{NetworkStream};
use super::types::{HttpMethod, Uri, HttpVersion};

pub trait Request {}


/// Represents the structure of an Http request, after parsing and validating the TCP stream of data
/// 
/// HttpRequest autovalidates the incoming Http request when a new stream reaches the server.
/// 
/// A is_valid() should represent this status // TODO 
/// TODO Docs
#[derive(Debug)]
pub struct HttpRequest {
    pub verb: HttpMethod,
    pub uri: Uri,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl Request for HttpRequest {}

impl HttpRequest {

    pub fn new<'a, T: NetworkStream>(stream: &'a mut T) -> HttpRequest {

        let mut buffer = [0; 1024];  // TODO Handle the buffer accordingly to the incoming request
                                                // or let the user handle the MAX allocated value
        stream.read(&mut buffer).unwrap();  // TODO Handle the possible error on io::Write
        let request_payload = String::from_utf8_lossy(&buffer[..]);  // let binding for longer live the slices

        let sp = request_payload.split("\r\n")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        
        let only_read = sp.clone();
        let rc = Rc::new(RefCell::new(sp.into_iter()));
        
        // Take the first element on the iterator, that contains the verb, uri and http version of the request
        let (verb, uri, version) = Self::parse_verb_uri_version(Rc::clone(&rc));
        // Then, we get (not consume) the last element on the stream, that is the body of the Http request
        let body = Self::parse_http_request_body(&only_read.last().unwrap());
        // Finally, we consume the intermediate elements, the remaining ones in the iterator
        let headers = Self::parse_http_request_headers(Rc::clone(&rc));

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
    fn parse_verb_uri_version<'b>(payload: Rc<RefCell<std::vec::IntoIter<String>>>) -> (HttpMethod, Uri, HttpVersion) {
        let method_uri_version = payload.borrow_mut().next()
            .expect("Something wrong happened getting verb-uri-version");
        let mut sp = method_uri_version.split_ascii_whitespace();  // let binding for the temporary

        (
            Self::parse_http_method(sp.next().unwrap()),
            Self::parse_uri(sp.next().unwrap()),
            Self::parse_http_version(sp.next().unwrap())
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
        // TODO Implement the URI parser based on a future generated Router type
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
    fn parse_http_request_headers<'b>(sp: Rc<RefCell<std::vec::IntoIter<String>>>) -> HashMap<String, String> {
        
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut iter = sp.borrow_mut();
            
        loop {
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
            } else { break; }
        }

        println!("\n HEADERS: {:?}", &headers);
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
        assert_eq!(uri.uri, *&MOCKED_PAYLOAD[1]);  // TODO Still dummy 'cause not fully impl the Uri parser
    }

    #[test]
    fn test_http_version_parser() {
        let http_version = HttpRequest::parse_http_version(&MOCKED_PAYLOAD[2]);
        assert_eq!(http_version, HttpVersion::V1_1)
    }

    #[test]
    fn test_http_headers_parser() {
        let http_headers = HttpRequest::parse_http_request_headers(
            Rc::new(
                RefCell::new(
                    MOCKED_PAYLOAD[3].split("\r\n")
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .into_iter()
                )
            )
        );
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