use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        BearerToken, CharacterSchema, CraftSkillSchema, MonsterSchema, PaginatedResponseSchema,
        ResourceSchema, ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Name(String);

#[derive(TypedBuilder)]
pub struct CreateCharacterRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into))]
    skin: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/create_character_characters_create_post>
pub fn create_character(
    CreateCharacterRequest {
        bearer_token,
        name,
        skin,
    }: CreateCharacterRequest,
) -> Result<EncodedRequest<CreateCharacterRequest>, crate::Error> {
    let name = Name::try_new(name)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_static("/characters/create"),
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (AUTHORIZATION, HeaderValue::from_str(&bearer_token.0)?),
        ]),
        content: serde_json::to_vec(&json!({
            "name": name,
            "skin": skin
        }))?,
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);

#[derive(TypedBuilder)]
pub struct GetAllCharactersRequest {
    #[builder(default = 1)]
    page: u32,
    #[builder(default = 50)]
    size: u32,
    #[builder(default)]
    sort: Option<CraftSkillSchema>,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_characters_characters__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_characters(
    GetAllCharactersRequest { page, size, sort }: GetAllCharactersRequest,
) -> Result<EncodedRequest<GetAllCharactersRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    let mut query = Vec::new();
    if let Some(sort) = sort {
        query.push(format!("sort={sort}"));
    }
    let path = format!("/characters/?page={page}&size={size}&{}", query.join("&"));

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&path)?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllCharactersRequest> {
    type Response = PaginatedResponseSchema<CharacterSchema>;
}

#[derive(TypedBuilder)]
pub struct GetCharacterRequest {
    #[builder(setter(into))]
    name: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_character_characters__name__get>
#[tracing::instrument(level = "trace")]
pub fn get_character(
    GetCharacterRequest { name }: GetCharacterRequest,
) -> Result<EncodedRequest<GetCharacterRequest>, crate::Error> {
    let name = Name::try_new(name)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/characters/{name}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetCharacterRequest> {
    type Response = ResponseSchema<CharacterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn create_character_should_work_with_valid_input(
            name in "[a-zA-Z0-9_-]+"
                .prop_filter(
                    "name must be at least 3 characters and at most 12",
                    |n| n.len() >= 3 && n.len() <= 12,
                ),
            skin in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::CreateCharacterRequest::builder()
                .bearer_token(BearerToken("a valid token".to_string()))
                .name(name)
                .skin(skin)
                .build();
            assert!(super::create_character(request).is_ok());
        }
        #[test]
        fn get_all_characters_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            let request = super::GetAllCharactersRequest::builder()
                .page(page)
                .size(size)
                .build();
            assert!(super::get_all_characters(request).is_ok());
        }
        #[test]
        fn get_character_should_work_with_valid_input(
            name in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetCharacterRequest::builder()
                .name(name)
                .build();
            assert!(super::get_character(request).is_ok());
        }
    }
}
