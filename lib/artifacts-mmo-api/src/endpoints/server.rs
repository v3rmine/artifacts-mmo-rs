use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};

use crate::{rate_limits::NO_RATE_LIMIT, EncodedRequest};

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[tracing::instrument(level = "trace")]
pub fn get_status() -> EncodedRequest {
    EncodedRequest {
        path: PathAndQuery::from_static("/"),
        method: Method::GET,
        headers: HeaderMap::from_iter([(ACCEPT, HeaderValue::from_static("application/json"))]),
        content: Vec::new(),
        rate_limit: NO_RATE_LIMIT,
    }
}
