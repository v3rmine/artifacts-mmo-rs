use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        MonsterSchema, PaginatedResponseSchema, ResourceSchema, ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);
#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Drop(String);

struct GetAllMonstersRequest;
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_monsters_monsters__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_monsters(
    drop: Option<&str>,
    max_level: Option<u32>,
    min_level: Option<u32>,
    page: u32,
    size: u32,
) -> Result<EncodedRequest<GetAllMonstersRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    let mut query = Vec::new();
    if let Some(drop) = drop {
        let drop = Drop::try_new(drop)
            .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
            .into_inner();
        query.push(format!("drop={drop}"));
    }
    if let Some(max_level) = max_level {
        query.push(format!("max_level={max_level}"));
    }
    if let Some(min_level) = min_level {
        query.push(format!("min_level={min_level}"));
    }
    let path = format!("/monsters/?page={page}&size={size}&{}", query.join("&"));

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&path)?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllMonstersRequest> {
    type Response = PaginatedResponseSchema<MonsterSchema>;
}

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

struct GetMonsterRequest;
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_monster_monsters__code__get>
#[tracing::instrument(level = "trace")]
pub fn get_monster(code: &str) -> Result<EncodedRequest<GetMonsterRequest>, crate::Error> {
    let code = Code::try_new(code)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/monsters/{code}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetMonsterRequest> {
    type Response = ResponseSchema<MonsterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_all_monsters_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            assert!(super::get_all_monsters(None, None, None, page, size).is_ok());
        }
        #[test]
        fn get_monster_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            assert!(super::get_monster(&code).is_ok());
        }
    }
}
