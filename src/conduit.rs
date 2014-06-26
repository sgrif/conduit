extern crate semver;

use std::collections::HashMap;
use std::io::net::ip::IpAddr;

pub enum Scheme {
    Http,
    Https
}

pub enum Host<'a> {
    HostName(&'a str),
    HostIp(IpAddr)
}

pub enum Method<'a> {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Other(&'a str)
}

pub trait Request {
    /// The version of HTTP being used
    fn http_version(&self) -> semver::Version;

    /// The version of the conduit spec being used
    fn conduit_version(&self) -> semver::Version;

    /// The request method, such as GET, POST, PUT, DELETE or PATCH
    fn method<'a>(&'a self) -> Method<'a>;

    /// The scheme part of the request URL
    fn scheme(&self) -> Scheme;

    /// The host part of the requested URL
    fn host<'a>(&'a self) -> Host<'a>;

    /// The initial part of the request URL's path that corresponds
    /// to a virtual root. This allows an application to have a
    /// virtual location that consumes part of the path.
    fn virtual_root<'a>(&'a self) -> Option<&'a str>;

    /// The remainder of the path.
    fn path<'a>(&'a self) -> &'a str;

    /// The portion of the request URL that follows the "?"
    fn query_string<'a>(&'a self) -> Option<&'a str>;

    /// The remote IP address of the client or the last proxy that
    /// sent the request.
    fn remote_ip(&self) -> IpAddr;

    /// The byte-size of the body, if any
    fn content_length(&self) -> Option<uint>;

    /// A Reader for the body of the request
    fn body<'a>(&'a mut self) -> &'a mut Reader;
}

pub struct Response {
    /// The status code as a tuple of the return code and status string
    pub status: (uint, &'static str),

    /// A Map of the headers
    pub headers: HashMap<String, String>,

    /// A Writer for body of the response
    pub body: Box<Reader + Send>
}

pub type Handler = fn(&mut Request) -> Response;
