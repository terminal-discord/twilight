#![allow(clippy::wildcard_imports)]

pub mod gateway;
pub mod shard;

mod dispatch;
mod kind;

pub use self::{
    dispatch::{DispatchEvent, DispatchEventWithTypeDeserializer},
    gateway::{GatewayEvent, GatewayEventDeserializer, GatewayEventDeserializerOwned},
    kind::EventType,
};

use self::shard::*;
use super::payload::*;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Any type of event that a shard emits.
///
/// This brings together all of the types of [`DispatchEvent`]s,
/// [`GatewayEvent`]s, and [`ShardEvent`]s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// A user was banned from a guild.
    BanAdd(BanAdd),
    /// A user's ban from a guild was removed.
    BanRemove(BanRemove),
    /// A channel was created.
    ChannelCreate(ChannelCreate),
    /// A channel was deleted.
    ChannelDelete(ChannelDelete),
    /// A channel's pins were updated.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// A channel was updated.
    ChannelUpdate(ChannelUpdate),
    /// A heartbeat was sent to or received from the gateway.
    GatewayHeartbeat(u64),
    /// A heartbeat acknowledgement was received from the gateway.
    GatewayHeartbeatAck,
    /// A "hello" packet was received from the gateway.
    GatewayHello(u64),
    /// A shard's session was invalidated.
    ///
    /// `true` if resumeable. If not, then the shard must do a full reconnect.
    GatewayInvalidateSession(bool),
    /// The gateway is indicating to perform a reconnect.
    GatewayReconnect,
    /// Undocumented event, should be ignored
    GiftCodeUpdate,
    /// A guild was created.
    GuildCreate(Box<GuildCreate>),
    /// A guild was deleted or the current user was removed from a guild.
    GuildDelete(Box<GuildDelete>),
    /// A guild's emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// A guild's integrations were updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// A guild was updated.
    GuildUpdate(Box<GuildUpdate>),
    /// A invite was made.
    InviteCreate(Box<InviteCreate>),
    /// A invite was deleted.
    InviteDelete(InviteDelete),
    /// A user was added to a guild.
    MemberAdd(Box<MemberAdd>),
    /// A set of guild members has been updated.
    MemberListUpdate(Box<MemberListUpdate>),
    /// A user was removed from a guild.
    MemberRemove(MemberRemove),
    /// A user's member object in a guild was updated.
    MemberUpdate(Box<MemberUpdate>),
    /// A chunk of members were received from the gateway.
    MemberChunk(MemberChunk),
    /// A message was created in a channel.
    MessageCreate(Box<MessageCreate>),
    /// A message was deleted in a channel.
    MessageDelete(MessageDelete),
    /// Multiple messages were deleted in a channel.
    MessageDeleteBulk(MessageDeleteBulk),
    /// A message was updated in a channel.
    MessageUpdate(Box<MessageUpdate>),
    /// A user's active presence (such as game or online status) was updated.
    PresenceUpdate(Box<PresenceUpdate>),
    /// Multiple presences outside of a guild were updated.
    ///
    /// For bots this is always empty and useless.
    PresencesReplace,
    /// A reaction was added to a message.
    ReactionAdd(Box<ReactionAdd>),
    /// A reaction was removed from a message.
    ReactionRemove(Box<ReactionRemove>),
    /// All reactions were removed from a message.
    ReactionRemoveAll(ReactionRemoveAll),
    /// All instances of a given emoji from the reactions of a message were
    /// removed.
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    /// A shard is now "ready" and fully connected.
    Ready(Box<Ready>),
    /// A shard has successfully resumed.
    Resumed,
    /// A role was created in a guild.
    RoleCreate(RoleCreate),
    /// A role was deleted in a guild.
    RoleDelete(RoleDelete),
    /// A role was updated in a guild.
    RoleUpdate(RoleUpdate),
    /// A shard is now in a connected stage after being fully connected to the
    /// gateway.
    ShardConnected(Connected),
    /// A shard is now in a connecting stage after starting to connect to the
    /// gateway.
    ShardConnecting(Connecting),
    /// A shard is now in a disconnected stage after the connection was closed.
    ShardDisconnected(Disconnected),
    /// A shard is now in a identifying stage after starting a new session.
    ShardIdentifying(Identifying),
    /// A shard is now in a reconnecting stage after a disconnect or session was
    /// ended.
    ShardReconnecting(Reconnecting),
    /// A payload of bytes came in through the shard's connection.
    ShardPayload(Payload),
    /// A shard is now in a Resuming stage after a disconnect.
    ShardResuming(Resuming),
    /// A user started typing in a channel.
    TypingStart(Box<TypingStart>),
    /// A guild is now unavailable.
    UnavailableGuild(UnavailableGuild),
    /// The current user was updated.
    UserUpdate(UserUpdate),
    /// A voice server update was sent.
    VoiceServerUpdate(VoiceServerUpdate),
    /// A voice state in a voice channel was updated.
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    /// A webhook was updated.
    WebhooksUpdate(WebhooksUpdate),
}

impl Event {
    pub fn kind(&self) -> EventType {
        match self {
            Self::BanAdd(_) => EventType::BanAdd,
            Self::BanRemove(_) => EventType::BanRemove,
            Self::ChannelCreate(_) => EventType::ChannelCreate,
            Self::ChannelDelete(_) => EventType::ChannelDelete,
            Self::ChannelPinsUpdate(_) => EventType::ChannelPinsUpdate,
            Self::ChannelUpdate(_) => EventType::ChannelUpdate,
            Self::GatewayHeartbeat(_) => EventType::GatewayHeartbeat,
            Self::GatewayHeartbeatAck => EventType::GatewayHeartbeatAck,
            Self::GatewayHello(_) => EventType::GatewayHello,
            Self::GatewayInvalidateSession(_) => EventType::GatewayInvalidateSession,
            Self::GatewayReconnect => EventType::GatewayReconnect,
            Self::GiftCodeUpdate => EventType::GiftCodeUpdate,
            Self::GuildCreate(_) => EventType::GuildCreate,
            Self::GuildDelete(_) => EventType::GuildDelete,
            Self::GuildEmojisUpdate(_) => EventType::GuildEmojisUpdate,
            Self::GuildIntegrationsUpdate(_) => EventType::GuildIntegrationsUpdate,
            Self::GuildUpdate(_) => EventType::GuildUpdate,
            Self::InviteCreate(_) => EventType::InviteCreate,
            Self::InviteDelete(_) => EventType::InviteDelete,
            Self::MemberAdd(_) => EventType::MemberAdd,
            Self::MemberListUpdate(_) => EventType::MemberListUpdate,
            Self::MemberRemove(_) => EventType::MemberRemove,
            Self::MemberUpdate(_) => EventType::MemberUpdate,
            Self::MemberChunk(_) => EventType::MemberChunk,
            Self::MessageCreate(_) => EventType::MessageCreate,
            Self::MessageDelete(_) => EventType::MessageDelete,
            Self::MessageDeleteBulk(_) => EventType::MessageDeleteBulk,
            Self::MessageUpdate(_) => EventType::MessageUpdate,
            Self::PresenceUpdate(_) => EventType::PresenceUpdate,
            Self::PresencesReplace => EventType::PresencesReplace,
            Self::ReactionAdd(_) => EventType::ReactionAdd,
            Self::ReactionRemove(_) => EventType::ReactionRemove,
            Self::ReactionRemoveAll(_) => EventType::ReactionRemoveAll,
            Self::ReactionRemoveEmoji(_) => EventType::ReactionRemoveEmoji,
            Self::Ready(_) => EventType::Ready,
            Self::Resumed => EventType::Resumed,
            Self::RoleCreate(_) => EventType::RoleCreate,
            Self::RoleDelete(_) => EventType::RoleDelete,
            Self::RoleUpdate(_) => EventType::RoleUpdate,
            Self::ShardConnected(_) => EventType::ShardConnected,
            Self::ShardConnecting(_) => EventType::ShardConnecting,
            Self::ShardDisconnected(_) => EventType::ShardDisconnected,
            Self::ShardIdentifying(_) => EventType::ShardIdentifying,
            Self::ShardReconnecting(_) => EventType::ShardReconnecting,
            Self::ShardPayload(_) => EventType::ShardPayload,
            Self::ShardResuming(_) => EventType::ShardResuming,
            Self::TypingStart(_) => EventType::TypingStart,
            Self::UnavailableGuild(_) => EventType::UnavailableGuild,
            Self::UserUpdate(_) => EventType::UserUpdate,
            Self::VoiceServerUpdate(_) => EventType::VoiceServerUpdate,
            Self::VoiceStateUpdate(_) => EventType::VoiceStateUpdate,
            Self::WebhooksUpdate(_) => EventType::WebhooksUpdate,
        }
    }
}

impl From<Box<DispatchEvent>> for Event {
    fn from(event: Box<DispatchEvent>) -> Self {
        match *event {
            DispatchEvent::BanAdd(v) => Self::BanAdd(v),
            DispatchEvent::BanRemove(v) => Self::BanRemove(v),
            DispatchEvent::ChannelCreate(v) => Self::ChannelCreate(v),
            DispatchEvent::ChannelDelete(v) => Self::ChannelDelete(v),
            DispatchEvent::ChannelPinsUpdate(v) => Self::ChannelPinsUpdate(v),
            DispatchEvent::ChannelUpdate(v) => Self::ChannelUpdate(v),
            DispatchEvent::GiftCodeUpdate => Self::GiftCodeUpdate,
            DispatchEvent::GuildCreate(v) => Self::GuildCreate(v),
            DispatchEvent::GuildDelete(v) => Self::GuildDelete(v),
            DispatchEvent::GuildEmojisUpdate(v) => Self::GuildEmojisUpdate(v),
            DispatchEvent::GuildIntegrationsUpdate(v) => Self::GuildIntegrationsUpdate(v),
            DispatchEvent::InviteCreate(v) => Self::InviteCreate(v),
            DispatchEvent::InviteDelete(v) => Self::InviteDelete(v),
            DispatchEvent::MemberAdd(v) => Self::MemberAdd(v),
            DispatchEvent::MemberListUpdate(v) => Self::MemberListUpdate(v),
            DispatchEvent::MemberRemove(v) => Self::MemberRemove(v),
            DispatchEvent::MemberUpdate(v) => Self::MemberUpdate(v),
            DispatchEvent::MemberChunk(v) => Self::MemberChunk(v),
            DispatchEvent::RoleCreate(v) => Self::RoleCreate(v),
            DispatchEvent::RoleDelete(v) => Self::RoleDelete(v),
            DispatchEvent::RoleUpdate(v) => Self::RoleUpdate(v),
            DispatchEvent::GuildUpdate(v) => Self::GuildUpdate(v),
            DispatchEvent::MessageCreate(v) => Self::MessageCreate(v),
            DispatchEvent::MessageDelete(v) => Self::MessageDelete(v),
            DispatchEvent::MessageDeleteBulk(v) => Self::MessageDeleteBulk(v),
            DispatchEvent::MessageUpdate(v) => Self::MessageUpdate(v),
            DispatchEvent::PresenceUpdate(v) => Self::PresenceUpdate(v),
            DispatchEvent::PresencesReplace => Self::PresencesReplace,
            DispatchEvent::ReactionAdd(v) => Self::ReactionAdd(v),
            DispatchEvent::ReactionRemove(v) => Self::ReactionRemove(v),
            DispatchEvent::ReactionRemoveAll(v) => Self::ReactionRemoveAll(v),
            DispatchEvent::ReactionRemoveEmoji(v) => Self::ReactionRemoveEmoji(v),
            DispatchEvent::Ready(v) => Self::Ready(v),
            DispatchEvent::Resumed => Self::Resumed,
            DispatchEvent::TypingStart(v) => Self::TypingStart(v),
            DispatchEvent::UnavailableGuild(v) => Self::UnavailableGuild(v),
            DispatchEvent::UserUpdate(v) => Self::UserUpdate(v),
            DispatchEvent::VoiceServerUpdate(v) => Self::VoiceServerUpdate(v),
            DispatchEvent::VoiceStateUpdate(v) => Self::VoiceStateUpdate(v),
            DispatchEvent::WebhooksUpdate(v) => Self::WebhooksUpdate(v),
        }
    }
}

impl From<GatewayEvent> for Event {
    fn from(event: GatewayEvent) -> Self {
        match event {
            GatewayEvent::Dispatch(_, e) => Self::from(e),
            GatewayEvent::Heartbeat(interval) => Self::GatewayHeartbeat(interval),
            GatewayEvent::HeartbeatAck => Self::GatewayHeartbeatAck,
            GatewayEvent::Hello(interval) => Self::GatewayHello(interval),
            GatewayEvent::InvalidateSession(r) => Self::GatewayInvalidateSession(r),
            GatewayEvent::Reconnect => Self::GatewayReconnect,
        }
    }
}

impl From<ShardEvent> for Event {
    fn from(event: ShardEvent) -> Self {
        match event {
            ShardEvent::Connected(v) => Self::ShardConnected(v),
            ShardEvent::Connecting(v) => Self::ShardConnecting(v),
            ShardEvent::Disconnected(v) => Self::ShardDisconnected(v),
            ShardEvent::Identifying(v) => Self::ShardIdentifying(v),
            ShardEvent::Payload(v) => Self::ShardPayload(v),
            ShardEvent::Reconnecting(v) => Self::ShardReconnecting(v),
            ShardEvent::Resuming(v) => Self::ShardResuming(v),
        }
    }
}

/// An error that describes a failure to convert
/// from one event type to another.
#[derive(Debug)]
pub struct EventConversionError {
    /// The original event
    event: Event,
}

impl EventConversionError {
    pub fn new(event: Event) -> EventConversionError {
        Self { event }
    }

    /// Get the original event
    pub fn into_event(self) -> Event {
        self.event
    }
}

impl Display for EventConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "event variant failed to convert")
    }
}

impl Error for EventConversionError {}
