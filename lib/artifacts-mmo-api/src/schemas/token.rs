use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BearerToken(String);

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/generate_token_token__post>
#[derive(Debug, Clone, Serialize)]
pub struct TokenSchema {
    token: BearerToken,
}
