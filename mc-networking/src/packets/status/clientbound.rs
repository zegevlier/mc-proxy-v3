use mc_networking_macros::McEncodable;

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusResponse {
    pub json_response: String,
}

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct PingResponse {
    pub payload: i64,
}
