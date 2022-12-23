use mc_networking::{types::State, versions::Version};

#[derive(Debug, Clone, Copy)]
pub struct ConnInfo {
    pub state: State,
    pub protocol_version: Version,
}

impl Default for ConnInfo {
    fn default() -> Self {
        Self {
            state: State::Handshaking,
            protocol_version: Version::Unknown,
        }
    }
}
