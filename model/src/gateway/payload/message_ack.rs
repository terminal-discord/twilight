use crate::id::{ChannelId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageAck {
    pub message_id: MessageId,
    pub channel_id: ChannelId,
}
