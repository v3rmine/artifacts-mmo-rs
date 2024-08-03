use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        ItemSchema, MonsterSchema, PaginatedResponseSchema, ResourceSchema, ResponseSchema,
        SingleItemSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);
#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Drop(String);

#[derive(TypedBuilder)]
struct GetAllItemsRequest {
    #[builder(default = 1)]
    page: u32,
    #[builder(default = 50)]
    size: u32,
    #[builder(default, setter(into))]
    drop: Option<String>,
    #[builder(default)]
    max_level: Option<u32>,
    #[builder(default)]
    min_level: Option<u32>,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_items_items__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_items(
    GetAllItemsRequest {
        page,
        size,
        drop,
        max_level,
        min_level,
    }: GetAllItemsRequest,
) -> Result<EncodedRequest<GetAllItemsRequest>, crate::Error> {
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

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllItemsRequest> {
    type Response = PaginatedResponseSchema<ItemSchema>;
}

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

#[derive(TypedBuilder)]
struct GetItemRequest {
    #[builder(setter(into))]
    code: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_item_items__code__get>
#[tracing::instrument(level = "trace")]
pub fn get_item(
    GetItemRequest { code }: GetItemRequest,
) -> Result<EncodedRequest<GetItemRequest>, crate::Error> {
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

impl<'de> ParseResponse<'de> for EncodedRequest<GetItemRequest> {
    type Response = ResponseSchema<SingleItemSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_all_items_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            let request = super::GetAllItemsRequest::builder()
                .page(page)
                .size(size)
                .build();
            assert!(super::get_all_items(request).is_ok());
        }
        #[test]
        fn get_item_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetItemRequest::builder()
                .code(code)
                .build();
            assert!(super::get_item(request).is_ok());
        }
    }
}
