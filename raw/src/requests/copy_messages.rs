// use std::ops::Not;
//
// use crate::requests::*;
// use crate::types::*;
//
// /// Use this method to forward messages of any kind.
// #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
// #[must_use = "requests do nothing unless sent"]
// pub struct CopyMessages {
//     chat_id: ChatRef,
//     from_chat_id: ChatRef,
//     #[serde(skip_serializing_if = "Not::not")]
//     disable_notification: bool,
//     message_ids: Vec<MessageId>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // reply_markup: Option<ReplyMarkup>,
// }
//
// #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
// pub struct CopyMessagesResult {
//     pub message_ids: Vec<MessageId>,
// }
//
// impl Request for CopyMessages {
//     type Type = JsonRequestType<Self>;
//     // type Response = JsonIdResponse<serde_json::Value>;
//     type Response = JsonIdResponse<CopyMessagesResult>;
//
//     fn serialize(&self) -> Result<HttpRequest, Error> {
//         Self::Type::serialize(RequestUrl::method("copyMessages"), self)
//     }
// }
//
// impl CopyMessages {
//     pub fn new<M, F, T>(messages: Vec<M>, from: F, to: T) -> Self
//     where
//         M: ToMessageId,
//         F: ToChatRef,
//         T: ToChatRef,
//     {
//         CopyMessages {
//             chat_id: to.to_chat_ref(),
//             from_chat_id: from.to_chat_ref(),
//             disable_notification: false,
//             message_ids: messages.into_iter().map(|m| m.to_message_id()).collect(),
//             // reply_markup: None,
//         }
//     }
//
//     pub fn disable_notification(&mut self) -> &mut Self {
//         self.disable_notification = true;
//         self
//     }
// }
//
// /// Forward message.
// pub trait CanCopyMessages {
//     fn copy<T>(&self, to: T) -> CopyMessages
//     where
//         T: ToChatRef;
// }
//
// impl<M> CanCopyMessages for M
// where
//     M: ToMessageId + ToSourceChat,
// {
//     fn copy<T>(&self, to: T) -> CopyMessages
//     where
//         T: ToChatRef,
//     {
//         CopyMessages::new(self.to_message_ids(), self.to_source_chat(), to)
//     }
// }
