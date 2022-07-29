use std::net::TcpStream;

use crate::http::events::Request;

/// Defines a type that represents some network connection
pub trait NetworkStream: std::io::Read + std::io::Write {}

/// New type pattern for declare generic [`NetworkStream`] events
/// of [`std::net::TcpStream`] type
impl NetworkStream for TcpStream {}
impl Request for TcpStream {}



/// Some networking event that could be read in many ways and has a custom
/// parser that it's able to understand and translate the request to some
/// implementor type
// pub trait NetReadable<T>: Parseable {}
// impl Parseable for TcpStream {
//     fn parse<'a>(&self, content: &'a str) {
//         todo!()
//     }
// }
// impl<T> NetReadable<T> for TcpStream {}


/// TODO Docs
/// 
/// T where represents some kind of custom user-defined type for an net io
/// response (should be serializable, depending on the response type)
pub struct _Response<T: Request> {
    pub request: T
}

impl<T: Request> _Response<T> {
    /// Returns an Http response for a valid Http request
    pub fn _from_request<'a, R>(_request: &'a dyn Request) -> R {
        todo!()
    }
}