use crate::requests::*;
use crate::types::*;

#[derive(Serialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatRef,
    /// Unique identifier of the target user
    user_id: i64,
}

// impl Methods for ApproveChatJoinRequest {
//     fn endpoint(&self) -> String {
//         "ApproveChatJoinRequest".to_string()
//     }
// }
impl Request for ApproveChatJoinRequest {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("ApproveChatJoinRequest"), self)
    }
}

impl ApproveChatJoinRequest {
    pub fn new(chat_id: ChatRef, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }

    // pub chat_id: ChatRef,
    pub fn chat_id(&mut self, chat_id: ChatRef) -> &mut Self {
        self.chat_id = chat_id;
        self
    }
    // /// Unique identifier of the target user
    // pub user_id: i64,
    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.user_id = user_id;
        self
    }
}
