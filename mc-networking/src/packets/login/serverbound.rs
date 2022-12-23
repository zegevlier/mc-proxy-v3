use crate::{derive::McEncodable, types::Uuid};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct LoginStart {
    pub username: String,
    pub uuid: Option<Uuid>,
}
