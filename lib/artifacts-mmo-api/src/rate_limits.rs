//! Rate limits for the API
//! SOURCE: <https://docs.artifactsmmo.com/api_guide/rate_limits>

#[derive(Debug, Clone)]
pub struct RateLimit<'a> {
    pub id: &'static str,
    pub by: RateLimitBy,
    pub requests_limits: &'a [LimitPer],
}

#[derive(Debug, Clone)]
pub enum LimitPer {
    Hours(u64),
    Seconds(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum RateLimitBy {
    Ip,
}

pub const ACCOUNT_CREATION_RATE_LIMIT: RateLimit<'_> = RateLimit {
    id: "ACCOUNT_CREATION",
    by: RateLimitBy::Ip,
    requests_limits: &[LimitPer::Hours(50)],
};
pub const TOKEN_RATE_LIMIT: RateLimit<'_> = RateLimit {
    id: "TOKEN",
    by: RateLimitBy::Ip,
    requests_limits: &[LimitPer::Hours(50)],
};
pub const DATA_RATE_LIMIT: RateLimit<'_> = RateLimit {
    id: "DATA",
    by: RateLimitBy::Ip,
    requests_limits: &[LimitPer::Seconds(20), LimitPer::Hours(7200)],
};
pub const ACTIONS_RATE_LIMIT: RateLimit<'_> = RateLimit {
    id: "ACTIONS",
    by: RateLimitBy::Ip,
    requests_limits: &[LimitPer::Seconds(5), LimitPer::Hours(7200)],
};
