use std::fmt::{self};

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


pub trait InvalidDataOnRequest: fmt::Display + fmt::Debug {}


/// Raised when a non existent verb in a Http request it's found
pub struct UnknownVerb<'a> {
    wrong_verb: &'a str
}
impl<'a> InvalidDataOnRequest for UnknownVerb<'a> {}
impl<'a> fmt::Display for UnknownVerb<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred parsing the verb on the Http Request")
    }
}
impl<'a> fmt::Debug for UnknownVerb<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong verb: {}", self.wrong_verb)
    }
}


/// Raised when an Http request contains an invalid Http version
pub struct InvalidVersion<'a> {
    invalid_version: &'a str
}
impl<'a> InvalidDataOnRequest for InvalidVersion<'a> {}
impl<'a> fmt::Display for InvalidVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong http version: {}", self.invalid_version)
    }
}
impl<'a> fmt::Debug for InvalidVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong http version: {}", self.invalid_version)
    }
}
/// Raised when an Http request contains an not supported Http version
pub struct UnsupportedVersion<'a> {
    unsupported_version: &'a str
}
impl<'a> InvalidDataOnRequest for UnsupportedVersion<'a> {}
impl<'a> fmt::Display for UnsupportedVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong http version: {}", self.unsupported_version)
    }
}
impl<'a> fmt::Debug for UnsupportedVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong http version: {}", self.unsupported_version)
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
