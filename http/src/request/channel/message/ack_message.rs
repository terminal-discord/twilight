use crate::request::prelude::*;
use serde::Deserialize;
use twilight_model::id::{ChannelId, MessageId};

#[derive(Deserialize)]
pub struct AckMessageToken {
    pub token: String,
}

#[derive(Serialize)]
struct AckMessageFields {
    token: Option<String>,
}

/// Ack a message by [`ChannelId`] and [`MessageId`].
pub struct AckMessage<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, AckMessageToken>>,
    fields: AckMessageFields,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> AckMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        token: Option<String>,
    ) -> Self {
        Self {
            channel_id,
            fut: None,
            fields: AckMessageFields { token },
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::builder(Route::AckMessage {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
        })
        .json(&self.fields)?;

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

poll_req!(AckMessage<'_>, AckMessageToken);
