use crate::{Audio, Channel, Chat, ChatId, Group, Integer, Message, Supergroup, User};
use serde::{Deserialize, Deserializer};

// bak Eq, Ord, Hash
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ChatFullInfo {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    #[doc(hidden)]
    Unknown(RawChatFullInfo),
}

impl ChatFullInfo {
    pub fn id_i64(&self) -> i64 {
        self.id().into()
    }

    pub fn id(&self) -> ChatId {
        match *self {
            ChatFullInfo::Private(ref x) => x.id.into(),
            ChatFullInfo::Group(ref x) => x.id.into(),
            ChatFullInfo::Supergroup(ref x) => x.id.into(),
            ChatFullInfo::Channel(ref x) => x.id.into(),
            ChatFullInfo::Unknown(ref x) => x.id.into(),
        }
    }

    pub fn is_bot(&self) -> bool {
        match *self {
            ChatFullInfo::Private(ref x) => x.is_bot,
            ChatFullInfo::Group(_) => false,
            ChatFullInfo::Supergroup(_) => false,
            ChatFullInfo::Channel(_) => false,
            ChatFullInfo::Unknown(_) => false,
        }
    }
    pub fn title(&self) -> String {
        match *self {
            ChatFullInfo::Private(ref x) => format!(
                "{} {}",
                x.first_name.as_str(),
                x.last_name.clone().unwrap_or_default()
            ),
            ChatFullInfo::Group(ref x) => x.title.clone(),
            ChatFullInfo::Supergroup(ref x) => x.title.clone(),
            ChatFullInfo::Channel(ref x) => x.title.clone(),
            ChatFullInfo::Unknown(ref x) => x.title.clone().unwrap_or_default(),
        }
    }
    pub fn user_name(&self) -> String {
        match *self {
            ChatFullInfo::Private(ref x) => x.username.clone().unwrap_or_default(),
            ChatFullInfo::Group(ref _x) => "".to_string(),
            ChatFullInfo::Supergroup(ref x) => x.username.clone().unwrap_or_default(),
            ChatFullInfo::Channel(ref x) => x.username.clone().unwrap_or_default(),
            ChatFullInfo::Unknown(ref x) => x.username.clone().unwrap_or_default(),
        }
    }

    pub fn is_channel(&self) -> bool {
        match &self {
            ChatFullInfo::Channel(_) => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for ChatFullInfo {
    fn deserialize<D>(deserializer: D) -> Result<ChatFullInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: crate::RawChatFullInfo = Deserialize::deserialize(deserializer)?;

        Ok(match raw.type_.as_ref() {
            "private" => ChatFullInfo::Private(User {
                id: raw.id.into(),
                username: raw.username,
                first_name: raw.first_name.clone().unwrap_or("".to_string()),
                last_name: raw.last_name,
                is_bot: false,
                language_code: raw.language_code,
                is_forum: raw.is_forum,
            }),
            "group" => ChatFullInfo::Group(Group {
                id: raw.id.into(),
                // is_forum: raw.is_forum,
                // active_usernames: None,
                title: raw.title.clone().unwrap_or("".to_string()),
                all_members_are_administrators: raw
                    .all_members_are_administrators
                    .clone()
                    .unwrap_or(false),
                invite_link: raw.invite_link,
                is_forum: None,
                active_usernames: None,
            }),
            "supergroup" => ChatFullInfo::Supergroup(Supergroup {
                id: raw.id.into(),
                is_forum: raw.is_forum,
                active_usernames: None,
                title: raw.title.clone().unwrap_or("".to_string()),
                username: raw.username,
                invite_link: raw.invite_link,
                description: raw.description,
            }),
            "channel" => ChatFullInfo::Channel(Channel {
                id: raw.id.into(),
                is_forum: raw.is_forum,
                active_usernames: None,
                title: raw.title.clone().unwrap_or("".to_string()),
                username: raw.username,
                invite_link: raw.invite_link,
                description: raw.description,
            }),
            _ => ChatFullInfo::Unknown(raw),
        })
    }
}

/// This object represents a chat, directly mapped.
/// back-up  Eq, Ord, Hash,
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawChatFullInfo {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: String,

    /// Title, for supergroups, channels and group chats
    #[serde(default)]
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,

    //add by whr
    pub is_forum: Option<bool>,

    /// is_direct_messages	True	Optional. True,
    /// if the chat is the direct messages chat of a channel
    pub is_direct_messages: Option<bool>,
    /// Integer	Identifier of the accent color for the chat name and backgrounds of the chat photo,
    /// reply header,
    /// and link preview.
    /// See accent colors for more details.
    #[serde(default)]
    pub accent_color_id: i64,
    /// max_reaction_count	Integer	The maximum number of reactions that can be set on a message in the chat
    #[serde(default)]
    pub max_reaction_count: i64,
    /// active_usernames	Array of String	Optional. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    pub active_usernames: Option<Vec<String>>,

    /// personal_chat	Chat	Optional. For private chats, the personal channel of the user
    pub personal_chat: Option<Chat>,
    /// parent_chat	Chat	Optional. Information about the corresponding channel chat; for direct messages chats only
    pub parent_chat: Option<Chat>,

    /// background_custom_emoji_id	String	Optional. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    /// See custom emoji for more details.
    pub background_custom_emoji_id: Option<String>,
    /// profile_accent_color_id	Integer	Optional. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[serde(default)]
    pub profile_accent_color_id: i64,
    /// profile_background_custom_emoji_id	String	Optional. Custom emoji identifier of the emoji chosen by the chat for its profile background
    /// See custom emoji for more details.
    pub profile_background_custom_emoji_id: Option<String>,
    /// emoji_status_custom_emoji_id	String	Optional. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    /// See custom emoji for more details.
    /// See custom emoji for more details.
    pub emoji_status_custom_emoji_id: Option<String>,

    // pub emoji_status_expiration_date	Integer	Optional. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    pub emoji_status_expiration_date: Option<i64>,

    // pub bio	String	Optional. Bio of the other party in a private chat
    pub bio: Option<String>,

    // has_private_forwards	True	Optional. True, if privacy settings of the other party in the private chat allows to use tg://user?id=<user_id> links only in chats with the user
    pub has_private_forwards: Option<bool>,

    // has_restricted_voice_and_video_messages	True	Optional. True, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    pub has_restricted_voice_and_video_messages: Option<bool>,

    // join_to_send_messages	True	Optional. True, if users need to join the supergroup before they can send messages
    pub join_to_send_messages: Option<bool>,

    // join_by_request	True	Optional. True, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    pub join_by_request: Option<bool>,

    // description	String	Optional. Description, for groups, supergroups and channel chats
    pub description: Option<String>,

    // invite_link	String	Optional. Primary invite link, for groups, supergroups and channel chats
    pub invite_link: Option<String>,

    // pinned_message	Message	Optional. The most recent pinned message (by sending date)
    pub pinned_message: Option<Message>,

    // can_send_paid_media	True	Optional. True, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    pub can_send_paid_media: Option<bool>,

    // slow_mode_delay	Integer	Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    pub can_send_other_messages: Option<bool>,
    // unrestrict_boost_count	Integer	Optional. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    pub can_send_video_messages: Option<bool>,
    // message_auto_delete_time	Integer	Optional. The time after which all messages sent to the chat will be automatically deleted; in seconds
    pub can_send_audio_messages: Option<bool>,
    // has_aggressive_anti_spam_enabled	True	Optional. True, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    pub can_send_video_notes: Option<bool>,
    // has_hidden_members	True	Optional. True, if non-administrators can only get the list of bots and administrators in the chat
    pub can_send_media_groups: Option<bool>,
    // has_protected_content	True	Optional. True, if messages from the chat can't be forwarded to other chats
    pub can_send_polls: Option<bool>,
    // has_visible_history	True	Optional. True, if new chat members will have access to old messages; available only to chat administrators
    pub has_visible_history: Option<bool>,
    // sticker_set_name	String	Optional. For supergroups, name of the group sticker set
    pub sticker_set_name: Option<String>,

    // can_set_sticker_set	True	Optional. True, if the bot can change the group sticker set
    pub can_set_sticker_set: Option<bool>,

    // custom_emoji_sticker_set_name	String	Optional. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    pub custom_emoji_sticker_set_name: Option<String>,
    // linked_chat_id	Integer	Optional. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub linked_chat_id: Option<Integer>,
    // first_profile_audio	Audio	Optional. For private chats, the first audio added to the profile of the user
    pub first_profile_audio: Option<Audio>,
    // unique_gift_colors	UniqueGiftColors	Optional. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    pub last_profile_audio: Option<Audio>,
    // paid_message_star_count	Integer	Optional. The number of Telegram Stars a general user has to pay to send a message to the chat
    pub paid_message_star_count: Option<Integer>,
    // IETF language tag of the other party in a private chat
    pub language_code: Option<String>,
    // True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
    //

    // /// photo	ChatPhoto	Optional. Chat photo
    // pub photo: Option<ChatPhoto>,
    // /// birthdate	Birthdate	Optional. For private chats, the date of birth of the user
    // pub birthdate: Option<Birthdate>,
    // /// business_intro	BusinessIntro	Optional. For private chats with business accounts, the intro of the business
    // pub business_intro: Option<BusinessIntro>,
    // /// business_location	BusinessLocation	Optional. For private chats with business accounts, the location of the business
    // pub business_location: Option<BusinessLocation>,
    // /// business_opening_hours	BusinessOpeningHours	Optional. For private chats with business accounts, the opening hours of the business
    // pub business_opening_hours: Option<BusinessOpeningHours>,
    // /// available_reactions	Array of ReactionType	Optional. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    // pub available_reactions: Option<Vec<ReactionType>>,
    // // permissions	ChatPermissions	Optional. Default chat member permissions, for groups and supergroups
    // pub permissions: Option<ChatPermissions>,
    // // accepted_gift_types	AcceptedGiftTypes	Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    // pub accepted_gift_types: Option<AcceptedGiftTypes>,
    // // location	ChatLocation	Optional. For supergroups, the location to which the supergroup is connected
    // pub location: Option<ChatLocation>,
    // // rating	UserRating	Optional. For private chats, the rating of the user if any
    // pub rating: Option<UserRating>,
    //
}
