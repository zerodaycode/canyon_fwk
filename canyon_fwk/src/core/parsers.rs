use super::net::Request;

/// Type that defines an interface for parseable items
/// 
/// In the net context, usually a network event over a server
/// it's a good candidate to be parseable, e.g. an Http request
/// that flows to an Http server.
/// 
/// The trait has an associated type `Implementor`, that will be the return 
/// type after the `parse` implementation. 
/// 
/// Ex: Implementing this trait into the [`std::net::TcpStream`], using as implementor
/// the [`crate::http::events::HttpRequest`] type, will make an easy flow when a new
/// client is connected to the server, an we want to parse the incoming data listened
/// by the [`std::net::TcpListener`] to get a new instance of the `Implementor` type
/// defined as the associated type in this trait.
pub trait Parseable {
    type Implementor;
    fn parse<'a, T: Request>(stream: &'a mut T) -> Self::Implementor;
}
