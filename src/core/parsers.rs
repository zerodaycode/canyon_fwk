use crate::http::methods::HttpMethod;

/// Type for defines contracts over parseable items
/// 
/// In the net context, usually a network event over a server
/// it's a good candidate to be parseable, e.g. an Http request
/// that flows to an Http server.
pub trait Parseable {
    fn parse<'a>(&self, content: &'a str);
}
