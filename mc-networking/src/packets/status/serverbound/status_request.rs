use crate::{derive::McEncodable, packet};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusRequest {}

packet!(StatusRequest, 0x00);
