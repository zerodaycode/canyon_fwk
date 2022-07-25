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
/// More info on [Http Methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}