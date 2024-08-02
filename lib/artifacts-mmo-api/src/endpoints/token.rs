use std::str::FromStr;

use base64::Engine;
use http::{
    header::{ACCEPT, AUTHORIZATION},
    uri::PathAndQuery,
    HeaderMap, HeaderValue, Method,
};

use crate::{rate_limits::TOKEN_RATE_LIMIT, EncodedRequest};

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/generate_token_token__post>
#[tracing::instrument(level = "trace")]
pub fn generate_token(username: &str, password: &str) -> Result<EncodedRequest, crate::Error> {
    Ok(EncodedRequest {
        path: PathAndQuery::from_static("/token/"),
        method: Method::POST,
        headers: HeaderMap::from_iter([
            (ACCEPT, HeaderValue::from_static("application/json")),
            (
                AUTHORIZATION,
                HeaderValue::from_str(
                    &base64::prelude::BASE64_STANDARD.encode(format!("{username}:{password}")),
                )?,
            ),
        ]),
        content: Vec::new(),
        rate_limit: TOKEN_RATE_LIMIT,
    })
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn generate_token_should_not_panic(username in "\\PC*", password in "\\PC*") {
            assert!(super::generate_token(&username, &password).is_ok());
        }
    }
}
