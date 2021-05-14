use crate::{
    channel::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    gateway::presence::Presence,
    guild::Permissions,
    id::{GuildId, RoleId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberListUpdate {
    pub ops: Vec<MemberListUpdateOp>,
    pub online_count: u32,
    pub member_count: u32,
    pub id: MemberListId,
    pub guild_id: GuildId,
    pub groups: Vec<MemberListGroup>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "op")]
pub enum MemberListUpdateOp {
    Sync {
        range: Vec<u32>,
        items: Vec<MemberListItem>,
    },
    Invalidate {
        range: Vec<u32>,
    },
    Update {
        item: MemberListItem,
        index: u32,
    },
    Delete {
        index: u32,
    },
    Insert {
        item: MemberListItem,
        index: u32,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MemberListItem {
    Group(MemberListGroup),
    Member(Box<MemberListMember>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberListMember {
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
pub struct MemberListGroup {
    pub id: GroupId,
    pub count: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum MemberListId {
    #[serde(with = "stringy::everyone")]
    Everyone,
    MemberListId(PermissionsHash),
}

impl MemberListId {
    pub fn from_overwrites(
        everyone_perms: Permissions,
        overwrites: &[PermissionOverwrite],
    ) -> Self {
        if everyone_perms.contains(Permissions::VIEW_CHANNEL)
            && !overwrites
                .iter()
                .any(|o| o.deny.contains(Permissions::VIEW_CHANNEL))
        {
            return MemberListId::Everyone;
        }

        let mut hash_input = Vec::new();
        for perm in overwrites {
            let id = match perm.kind {
                PermissionOverwriteType::Member(user_id) => user_id.0,
                PermissionOverwriteType::Role(role_id) => role_id.0,
            };
            if perm.allow.contains(Permissions::VIEW_CHANNEL) {
                hash_input.push(format!("allow:{}", id))
            } else if perm.deny.contains(Permissions::VIEW_CHANNEL) {
                hash_input.push(format!("deny:{}", id))
            }
        }

        hash_input.sort();

        if hash_input.is_empty() {
            return MemberListId::Everyone;
        }
        MemberListId::MemberListId(PermissionsHash(
            murmur3::murmur3_32(&mut std::io::Cursor::new(hash_input.join(",")), 0)
                .expect("mmh3 should not fail"),
        ))
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct PermissionsHash(#[serde(with = "string_u32")] pub u32);

impl From<u32> for PermissionsHash {
    fn from(id: u32) -> Self {
        PermissionsHash(id)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum GroupId {
    #[serde(with = "stringy::online")]
    Online,
    #[serde(with = "stringy::offline")]
    Offline,
    RoleId(RoleId),
}

macro_rules! named_unit_variant {
    ($variant:ident) => {
        pub mod $variant {
            pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(stringify!($variant))
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = ();
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(concat!("\"", stringify!($variant), "\""))
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == stringify!($variant) {
                            Ok(())
                        } else {
                            Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
                        }
                    }
                }
                deserializer.deserialize_str(V)
            }
        }
    };
}

mod stringy {
    named_unit_variant!(everyone);
    named_unit_variant!(online);
    named_unit_variant!(offline);
}

pub(crate) mod string_u32 {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        marker::PhantomData,
    };

    struct IdVisitor<T: From<u32>>(PhantomData<T>);

    impl<'de, T: From<u32>> Visitor<'de> for IdVisitor<T> {
        type Value = T;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("string u32")
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map(T::from).map_err(DeError::custom)
        }
    }

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T: From<u32>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}
