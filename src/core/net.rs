use std::net::TcpStream;

use super::parsers::Parseable;

/// Defines a type that represents some network connection
pub trait NetworkStream: std::io::Read + std::io::Write {}

/// New type pattern for declare generic [`NetworkStream`] events
/// of [`std::net::TcpStream`] type
impl NetworkStream for TcpStream {}


pub trait Request<T>: NetworkStream + Parseable {}


/// TODO Docs
/// 
/// T where represents some kind of custom user-defined type for an net io
/// response (should be serializable, depending on the response type)
pub struct Response<T: Request<T>> {
    pub request: T
}

impl<T: Request<T>> Response<T> {
    /// Returns an Http response for a valid Http request
    pub fn from_request<Res>(request: impl Request<T>) -> Res {
        todo!()
    }
}