use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseSchema<T> {
    data: T,
}
