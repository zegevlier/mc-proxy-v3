use crate::derive::McEncodable;

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusRequest {}

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct PingRequest {
    pub payload: i64,
}
