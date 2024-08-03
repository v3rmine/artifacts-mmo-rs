use std::{marker::PhantomData, str::FromStr};

use base64::Engine;
use http::{
    header::{ACCEPT, AUTHORIZATION},
    uri::PathAndQuery,
    HeaderMap, HeaderValue, Method,
};

use crate::{
    helpers::ACCEPT_JSON, rate_limits::TOKEN_RATE_LIMIT, schemas::TokenSchema, EncodedRequest,
    ParseResponse,
};

struct GenerateTokenRequest;
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/generate_token_token__post>
#[tracing::instrument(level = "trace")]
pub fn generate_token(
    username: &str,
    password: &str,
) -> Result<EncodedRequest<GenerateTokenRequest>, crate::Error> {
    Ok(EncodedRequest {
        path: PathAndQuery::from_static("/token/"),
        method: Method::POST,
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(
                    &base64::prelude::BASE64_STANDARD.encode(format!("{username}:{password}")),
                )?,
            ),
        ]),
        content: Vec::new(),
        rate_limit: TOKEN_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GenerateTokenRequest> {
    type Response = TokenSchema;
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
