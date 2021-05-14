use crate::channel::ChannelType;
use crate::guild::PartialMember;
use crate::id::UserId;
use crate::user::User;
use crate::{
    guild::GuildStatus,
    id::{ChannelId, MessageId},
    user::CurrentUser,
};
use serde::{Deserialize, Serialize};

/// The last read message id and optional message count in a channel
#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadState {
    pub mention_count: Option<usize>,
    pub last_message_id: MessageId,
    pub id: ChannelId,
}

#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadStateWrapper {
    pub version: u64,
    pub partial: bool,
    pub entries: Vec<ReadState>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialPrivateChannel {
    pub id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub recipient_ids: Vec<UserId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub users: Vec<User>,
    pub guilds: Vec<GuildStatus>,
    pub private_channels: Vec<PartialPrivateChannel>,
    pub merged_members: Vec<Vec<PartialMember>>,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    pub read_state: ReadStateWrapper,
    #[serde(rename = "v")]
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::{ReadStateWrapper, Ready};
    use crate::{
        guild::{GuildStatus, UnavailableGuild},
        id::{GuildId, UserId},
        user::CurrentUser,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_ready() {
        let guilds = vec![
            GuildStatus::Offline(UnavailableGuild {
                id: GuildId(1),
                unavailable: true,
            }),
            GuildStatus::Offline(UnavailableGuild {
                id: GuildId(2),
                unavailable: true,
            }),
        ];

        let ready = Ready {
            users: vec![],
            guilds,
            private_channels: vec![],
            merged_members: vec![],
            session_id: "foo".to_owned(),
            shard: Some([4, 7]),
            user: CurrentUser {
                avatar: None,
                bot: false,
                discriminator: "1212".to_owned(),
                email: None,
                flags: None,
                id: UserId(3),
                locale: None,
                mfa_enabled: false,
                name: "bar".to_owned(),
                premium_type: None,
                public_flags: None,
                verified: None,
            },
            read_state: ReadStateWrapper {
                version: 1,
                partial: false,
                entries: vec![],
            },
            version: 8,
        };

        serde_test::assert_tokens(
            &ready,
            &[
                Token::Struct {
                    name: "Ready",
                    len: 9,
                },
                Token::Str("users"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("guilds"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("private_channels"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("merged_members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("session_id"),
                Token::Str("foo"),
                Token::Str("shard"),
                Token::Some,
                Token::Tuple { len: 2 },
                Token::U64(4),
                Token::U64(7),
                Token::TupleEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "CurrentUser",
                    len: 6,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1212"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("mfa_enabled"),
                Token::Bool(false),
                Token::Str("username"),
                Token::Str("bar"),
                Token::StructEnd,
                Token::Str("read_state"),
                Token::Struct {
                    name: "ReadStateWrapper",
                    len: 3,
                },
                Token::Str("version"),
                Token::U64(1),
                Token::Str("partial"),
                Token::Bool(false),
                Token::Str("entries"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("v"),
                Token::U64(8),
                Token::StructEnd,
            ],
        );
    }
}
