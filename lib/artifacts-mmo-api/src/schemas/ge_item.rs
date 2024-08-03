use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GEItemResponse {
    data: Vec<GEItemSchema>,
    total: u32,
    page: u32,
    size: u32,
    pages: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GEItemSchema {
    code: String,
    stock: u32,
    sell_price: u32,
    buy_price: u32,
}
