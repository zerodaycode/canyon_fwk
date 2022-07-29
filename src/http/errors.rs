use std::fmt::{self};

pub trait InvalidDataOnRequest: fmt::Display + fmt::Debug {}


/// Raised when a non existent verb in a Http request it's found
pub struct UnknownVerb<'a> {
    pub wrong_verb: &'a str
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
    pub invalid_version: &'a str
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
