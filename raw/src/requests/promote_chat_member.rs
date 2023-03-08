use crate::requests::*;
use crate::types::*;

#[derive(Serialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatRef,
    /// Unique identifier of the target user
    user_id: i64,
    /// Pass True if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    /// Pass True if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    /// Pass True if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    /// Pass True if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    /// Pass True if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    /// Pass True if the administrator can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_video_chats: Option<bool>,
    /// Pass True if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    /// Pass True if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    /// Pass True if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    /// Pass True if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    /// Pass True if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
}

// impl Methods for PromoteChatMember {
//     fn endpoint(&self) -> String {
//         "promoteChatMember".to_string()
//     }
// }
impl Request for PromoteChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("promoteChatMember"), self)
    }
}

impl PromoteChatMember {
    pub fn new(chat_id: ChatRef, user_id: i64) -> Self {
        Self {
            chat_id,
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
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
    // pub is_anonymous: Option<bool>,
    pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    // /// Pass True if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    // pub can_manage_chat: Option<bool>,
    pub fn can_manage_chat(&mut self, can_manage_chat: bool) -> &mut Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }

    // /// Pass True if the administrator can create channel posts, channels only
    // pub can_post_messages: Option<bool>,
    pub fn can_post_messages(&mut self, can_post_messages: bool) -> &mut Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }
    // /// Pass True if the administrator can edit messages of other users and can pin messages, channels only
    // pub can_edit_messages: Option<bool>,
    pub fn can_edit_messages(&mut self, can_edit_messages: bool) -> &mut Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }
    // /// Pass True if the administrator can delete messages of other users
    // pub can_delete_messages: Option<bool>,
    pub fn can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }
    // /// Pass True if the administrator can manage video chats
    // pub can_manage_video_chats: Option<bool>,
    pub fn can_manage_video_chats(&mut self, can_manage_video_chats: bool) -> &mut Self {
        self.can_manage_video_chats = Some(can_manage_video_chats);
        self
    }
    // /// Pass True if the administrator can restrict, ban or unban chat members
    // pub can_restrict_members: Option<bool>,
    pub fn can_restrict_members(&mut self, can_restrict_members: bool) -> &mut Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }
    // /// Pass True if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    // pub can_promote_members: Option<bool>,
    pub fn can_promote_members(&mut self, can_promote_members: bool) -> &mut Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }
    // /// Pass True if the administrator can change chat title, photo and other settings
    // pub can_change_info: Option<bool>,
    pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
        self.can_change_info = Some(can_change_info);
        self
    }
    // /// Pass True if the administrator can invite new users to the chat
    // pub can_invite_users: Option<bool>,
    pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }
    // /// Pass True if the administrator can pin messages, supergroups only
    // pub can_pin_messages: Option<bool>,
    pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }
}
