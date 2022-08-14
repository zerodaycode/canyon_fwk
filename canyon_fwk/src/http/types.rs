use super::errors::{InvalidVersion, UnknownVerb, InvalidDataOnRequest};

/// Represents any valid Http method
/// 
/// HTTP defines a set of request methods to indicate the desired action 
/// to be performed for a given resource. 
/// Although they can also be nouns, these request methods are sometimes
/// referred to as HTTP verbs. 
/// 
/// Each of them implements a different semantic, but some common features
/// are shared by a group of them: e.g. a request method can be 
/// safe, idempotent, or cacheable.
/// 
/// More info on [Http Methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
    PUT,
    PATCH,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

impl HttpMethod {
    pub fn from_str<'a>(verb: &'a str) -> Result<Self, UnknownVerb> {
        match verb {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "HEAD" => Ok(Self::HEAD),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(UnknownVerb { wrong_verb: verb })
        }
    }
}


/// Defines the valid Http standards supported by this framework
#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    V1_1,
    V2
}

impl HttpVersion {
    pub fn from_str<'a>(version: &'a str) -> Result<HttpVersion, impl InvalidDataOnRequest + 'a> {
        match version {
            "HTTP/1.1" => Ok(Self::V1_1),
            "HTTP/2" => Ok(Self::V2),
            // "HTTP/1" => Err(UnsupportedVersion { unsupported_version: version} ), // TODO Not working with impl types
            _ => Err(InvalidVersion { invalid_version: version} )
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Uri {
    pub uri: String
}

impl Uri {
    pub fn new<'a>(uri: &'a str) -> Self {
        Self { uri: uri.to_string() }
    }
}
