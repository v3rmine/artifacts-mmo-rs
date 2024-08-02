pub mod endpoints;
pub mod rate_limits;
pub mod schemas;

mod errors;
pub use errors::Error;

use http::{uri::PathAndQuery, HeaderMap, Method, Request};

use self::rate_limits::RateLimit;

pub const API_VERSION: &str = "v1.3";
pub const API_BASE_URL: &str = "https://api.artifactsmmo.com/";

#[derive(Debug, Clone)]
pub struct EncodedRequest {
    pub method: Method,
    pub path: PathAndQuery,
    pub headers: HeaderMap,
    // Content as binary because the client doesn't need to know the format (sans-io style)
    pub content: Vec<u8>,
    // Rate limit is part of the API definition so we know it at comptime
    pub rate_limit: RateLimit<'static>,
}

impl TryFrom<EncodedRequest> for Request<Vec<u8>> {
    type Error = http::Error;

    fn try_from(value: EncodedRequest) -> Result<Self, Self::Error> {
        let request = Request::builder().method(&value.method).uri(value.path);

        let request = value
            .headers
            .iter()
            .fold(request, |request, (key, value)| request.header(key, value));

        request.body(value.content)
    }
}