// use std::collections::HashMap;
// use crate::{ChatRef, InputFileX, InputMedia, Multipart, ToMultipart};
// use crate::requests::*;
// use crate::types::*;
//
// /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct SendMediaGroup {
//     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//     pub chat_id: i64,
//     /// A JSON-serialized array describing messages to be sent, must include 2-10 items
//     #[serde(serialize_with = "serialize_input_media")]
//     pub media: Vec<InputMedia>,
//     /// Sends messages silently. Users will receive a notification with no sound.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub disable_notification: Option<bool>,
//     /// Protects the contents of the sent messages from forwarding and saving
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub protect_content: Option<bool>,
//     /// If the messages are a reply, ID of the original message
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub reply_to_message_id: Option<i64>,
//     /// Pass True if the message should be sent even if the specified replied-to message is not found
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub allow_sending_without_reply: Option<bool>,
// }
//
// /// SendMediaGroup serialize media field
// fn serialize_input_media<S>(input_media: &[InputMedia], s: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
// {
//     use serde::ser::SerializeSeq;
//     let mut seq = s.serialize_seq(Some(input_media.len()))?;
//     let mut idx = 0;
//     for elem in input_media {
//         seq.serialize_element(&(elem.prepare_input_media_param(idx)))?;
//         idx += 1;
//     }
//     seq.end()
// }
//
// impl SendMediaGroup {
//     pub fn new(chat_id: i64, media: Vec<InputMedia>) -> Self {
//         Self {
//             chat_id,
//             media,
//             disable_notification: None,
//             protect_content: None,
//             reply_to_message_id: None,
//             allow_sending_without_reply: None,
//         }
//     }
// }
//
//
// impl SendMediaGroup {
//     fn endpoint(&self) -> String {
//         "sendMediaGroup".to_string()
//     }
//     pub fn files(&self) -> HashMap<String, InputFileX> {
//         let mut result = HashMap::new();
//         let mut idx = 0;
//         for elem in self.media.clone() {
//             for (name, file) in elem.prepare_input_media_file(idx) {
//                 result.insert(name, file);
//             }
//             idx += 1;
//         }
//         result
//     }
// }
//
//
// impl ToMultipart for SendMediaGroup {
//
//     // chat_id,
//     // media,
//     // disable_notification: None,
//     // protect_content: None,
//     // reply_to_message_id: None,
//     // allow_sending_without_reply: None,
//
//     fn to_multipart(&self) -> Result<Multipart, Error> {
//         multipart_map! {
//             self,
//             (chat_id (i64));
//             (media (raw));
//             (disable_notification (raw), optional);
//             (protect_content (raw), optional);
//             (reply_to_message_id (raw), optional);
//             (allow_sending_without_reply (raw), optional);
//             (title (text), optional);
//             (thumb (raw), optional);
//             (reply_to_message_id (text), optional);
//             (disable_notification (text), when_true);
//             (reply_markup (json), optional);
//         }
//     }
// }
