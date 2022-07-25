use crate::http::{methods::HttpMethod, events::Request};

use super::net::NetworkStream;

/// Type for defines contracts over parseable items
/// 
/// In the net context, usually a network event over a server
/// it's a good candidate to be parseable, e.g. an Http request
/// that flows to an Http server.
pub trait Parseable {
    fn parse<'a, T>(stream: &'a mut T) -> &'a dyn Request
        where T: Parseable + Request;
}
