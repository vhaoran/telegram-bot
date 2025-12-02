use crate::requests::*;
use crate::types::*;

/// Use this method to export chat invite links.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateChatInviteLink {
    chat_id: ChatRef,
    /// Invite link name; 0-32 characters
    name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat
    /// simultaneously after joining the chat via this invite link;
    /// 1-99999
    member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators.
    /// If True, member_limit can't be specified
    creates_join_request: Option<bool>,
}

impl Request for CreateChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("CreateChatInviteLink"), self)
    }
}

impl CreateChatInviteLink {
    pub fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        CreateChatInviteLink {
            chat_id: chat.to_chat_ref(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    pub fn expire_date(&mut self, expire_date: i64) -> &mut Self {
        self.expire_date = Some(expire_date);
        self
    }
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn member_limit(&mut self, member_limit: i64) -> &mut Self {
        self.member_limit = Some(member_limit);
        self
    }

    pub fn creates_join_request(&mut self, creates_join_request: bool) -> &mut Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }
}

/// Export chat invite link.
pub trait CanCreateChatInviteLink {
    fn create_invite_link(&self) -> CreateChatInviteLink;
}

impl<C> CanCreateChatInviteLink for C
where
    C: ToChatRef,
{
    fn create_invite_link(&self) -> CreateChatInviteLink {
        CreateChatInviteLink::new(self)
    }
}
