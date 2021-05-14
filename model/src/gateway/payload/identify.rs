use super::update_status::UpdateStatusInfo;
use crate::gateway::opcode::OpCode;
use crate::gateway::presence::Status;
use crate::id::MessageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

impl Identify {
    pub fn new(info: IdentifyInfo) -> Self {
        Self {
            d: info,
            op: OpCode::Identify,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentifyInfo {
    pub token: String,
    pub capabilities: u32,
    pub properties: IdentifyProperties,
    pub presence: Option<UpdateStatusInfo>,
    pub compress: bool,
    pub client_state: IdentifyClientState,
}

impl IdentifyInfo {
    pub fn new(token: &str, properties: IdentifyProperties) -> Self {
        Self {
            token: token.to_owned(),
            capabilities: 61,
            properties,
            presence: Some(UpdateStatusInfo {
                status: Status::Online,
                since: Some(0),
                activities: Some(Vec::new()),
                afk: false,
            }),
            compress: false,
            client_state: IdentifyClientState::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentifyClientState {
    pub guild_hashes: HashMap<String, String>,
    pub highest_last_message_id: MessageId,
    pub read_state_version: u64,
    pub user_guild_settings_version: i64,
}

impl IdentifyClientState {
    pub fn default() -> Self {
        Self {
            guild_hashes: HashMap::new(),
            highest_last_message_id: MessageId(0),
            read_state_version: 0,
            user_guild_settings_version: -1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyProperties {
    os: String,
    browser: String,
    release_channel: String,
    client_version: String,
    os_version: String,
    os_arch: String,
    system_locale: String,
    client_build_number: u64,
    client_event_source: Option<String>,
}

impl IdentifyProperties {
    pub fn default_windows_desktop() -> Self {
        Self {
            os: "Mac OS X".into(),
            browser: "Discord Client".into(),
            release_channel: "stable".into(),
            client_version: "1.0.9001".into(),
            os_version: "10.0.19042".into(),
            os_arch: "x64".into(),
            system_locale: "en-US".into(),
            client_build_number: 84941,
            client_event_source: None,
        }
    }
}
