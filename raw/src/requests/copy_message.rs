use std::ops::Not;

use crate::requests::*;
use crate::types::*;

/// Use this method to forward messages of any kind.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CopyMessage {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    message_id: MessageId,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CopyMessageResult {
    pub message_id: Integer,
}

impl Request for CopyMessage {
    type Type = JsonRequestType<Self>;
    // type Response = JsonIdResponse<serde_json::Value>;
    type Response = JsonIdResponse<CopyMessageResult>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("copyMessage"), self)
    }
}

impl CopyMessage {
    pub fn new<M, F, T>(message: M, from: F, to: T) -> Self
    where
        M: ToMessageId,
        F: ToChatRef,
        T: ToChatRef,
    {
        CopyMessage {
            chat_id: to.to_chat_ref(),
            from_chat_id: from.to_chat_ref(),
            disable_notification: false,
            message_id: message.to_message_id(),
        }
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}

/// Forward message.
pub trait CanCopyMessage {
    fn copy<T>(&self, to: T) -> CopyMessage
    where
        T: ToChatRef;
}

impl<M> CanCopyMessage for M
where
    M: ToMessageId + ToSourceChat,
{
    fn copy<T>(&self, to: T) -> CopyMessage
    where
        T: ToChatRef,
    {
        CopyMessage::new(self.to_message_id(), self.to_source_chat(), to)
    }
}
