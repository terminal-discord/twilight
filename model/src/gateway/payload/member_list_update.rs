use crate::{
    gateway::presence::Presence,
    id::GuildId,
    id::{RoleId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberListUpdate {
    pub ops: Vec<MemberListUpdateOp>,
    pub online_count: u32,
    pub member_count: u32,
    // TODO(Noskcaj19): Replace with enum, variants unknown ("everyone", ...)
    pub id: String,
    pub guild_id: GuildId,
    pub groups: Vec<MemberListUpdateGroup>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "op")]
pub enum MemberListUpdateOp {
    Sync {
        // what is the purpose of this field?
        range: Vec<u32>,
        items: Vec<MemberListUpdateItem>,
    },
    Update {
        item: MemberListUpdateItem,
        // what is the purpose of this field?
        index: u32,
    },
    Delete {
        index: u32,
    },
    Insert {
        item: MemberListUpdateItem,
        index: u32,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MemberListUpdateItem {
    Group { id: String, count: u32 },
    Member(Box<MemberListUpdateMember>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberListUpdateMember {
    pub user: PartialUser,
    pub roles: Vec<RoleId>,
    pub presence: Presence,
    pub mute: bool,
    pub joined_at: String,
    pub hoisted_role: Option<RoleId>,
    pub deaf: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialUser {
    pub avatar: Option<String>,
    pub discriminator: String,
    pub id: UserId,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberListUpdateGroup {
    // TODO(Noskcaj19): Replace with enum, variants unknown ("online", "offline", id, ...)
    pub id: String,
    pub count: u32,
}
