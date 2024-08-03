use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{PaginatedResponseSchema, ResourceSchema},
    EncodedRequest, ParseResponse,
};

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);

struct GetAllResourcesRequest;
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_resources_resources__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_resources(
    page: u32,
    size: u32,
) -> Result<EncodedRequest<GetAllResourcesRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/resources/?page={page}&size={size}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllResourcesRequest> {
    type Response = PaginatedResponseSchema<ResourceSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_all_events_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50
        ) {
            assert!(super::get_all_resources(page, size).is_ok());
        }
    }
}
