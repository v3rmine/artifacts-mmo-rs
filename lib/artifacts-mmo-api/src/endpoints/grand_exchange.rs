use std::str::FromStr;

use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;

use crate::{helpers::ACCEPT_JSON, rate_limits::DATA_RATE_LIMIT, EncodedRequest};

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_ge_items_ge__get>
pub fn get_all_ge_items(page: u32, size: u32) -> Result<EncodedRequest, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/ge/?page={page}&size={size}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
    })
}

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_ge_item_ge__code__get>
pub fn get_ge_item(code: impl AsRef<str>) -> Result<EncodedRequest, crate::Error> {
    let code = Code::try_new(code.as_ref())
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/ge/{code}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
    })
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_all_ge_items_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50
        ) {
            assert!(super::get_all_ge_items(page, size).is_ok());
        }
        #[test]
        fn get_ge_item_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            assert!(super::get_ge_item(code).is_ok());
        }
    }
}
