use crate::requests::*;
use crate::types::*;
#[allow(unused_imports)]
use crate::{ChatRef, InputFileX, InputMedia, Multipart, ToMultipart};
#[allow(unused_imports)]
use std::collections::HashMap;

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMedia>,
    /// Sends messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the messages are a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

impl SendMediaGroup {
    pub fn new(chat_id: i64, media: Vec<InputMedia>) -> Self {
        Self {
            chat_id,
            media,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
}

impl ToMultipart for SendMediaGroup {
    // chat_id,
    // media,
    // disable_notification: None,
    // protect_content: None,
    // reply_to_message_id: None,
    // allow_sending_without_reply: None,
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (media (json));
            (disable_notification (text), optional);
            (protect_content (text), optional);
            (reply_to_message_id (text), optional);
            (allow_sending_without_reply (text), optional);
        }
    }
}

impl Request for SendMediaGroup {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Vec<Message>>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendMediaGroup"), self)
    }
}
