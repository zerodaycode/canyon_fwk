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
    pub fn from_str<'a>(verb: &str) -> Result<Self, UnknownVerb> {
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

/// Represents an error when a non existent verb in a Http request it's found
pub struct UnknownVerb<'a> {
    wrong_verb: &'a str
}
impl fmt::Display for UnknownVerb<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred parsing the verb on the Http Request")
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for UnknownVerb<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong verb: {}", self.wrong_verb)
    }
}

#[derive(Debug)]
pub struct Uri {
    pub uri: String
}

impl Uri {
    pub fn new<'a>(uri: &'a str) -> Self {
        Self { uri: uri.to_string() }
    }
}
