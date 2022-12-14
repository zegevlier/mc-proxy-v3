use mc_networking::versions::Version;

pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

pub struct ConnInfo {
    pub state: State,
    pub protocol_version: Option<Version>,
}

impl Default for ConnInfo {
    fn default() -> Self {
        Self {
            state: State::Handshaking,
            protocol_version: None,
        }
    }
}
