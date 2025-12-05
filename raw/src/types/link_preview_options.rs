use crate::{ChatId, ChatRef, MessageEntity};

#[derive(Serialize, Deserialize, Default, PartialEq, PartialOrd, Debug, Clone)]
// #[serde(untagged)]
pub struct LinkPreviewOptions {
    /// is_disabled	Boolean	Optional. True,
    /// if the link preview is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    is_disabled: Option<bool>,
    /// url	String	Optional. URL to use for the link preview.
    /// If empty, then the first URL found in the message text will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// prefer_small_media	Boolean	Optional. True,
    /// if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_small_media: Option<bool>,
    /// prefer_large_media	Boolean	Optional. True,
    /// if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_large_media: Option<bool>,
    // show_above_text	Boolean	Optional. True, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(skip_serializing_if = "Option::is_none")]
    show_above_text: Option<bool>,
}

impl LinkPreviewOptions {
    pub fn new() -> Self {
        Self {
            is_disabled: None,
            url: None,
            prefer_small_media: None,
            prefer_large_media: None,
            show_above_text: None,
        }
    }
    pub fn is_disabled(&mut self, is_disabled: bool) -> &mut Self {
        self.is_disabled = Some(is_disabled);
        self
    }
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }
    pub fn prefer_small_media(&mut self, prefer_small_media: bool) -> &mut Self {
        self.prefer_small_media = Some(prefer_small_media);
        self
    }
    pub fn prefer_large_media(&mut self, prefer_large_media: bool) -> &mut Self {
        self.prefer_large_media = Some(prefer_large_media);
        self
    }
    pub fn show_above_text(&mut self, show_above_text: bool) -> &mut Self {
        self.show_above_text = Some(show_above_text);
        self
    }
}

//-------------------------------------
#[derive(Deserialize, Serialize, Default, PartialEq, PartialOrd, Debug, Clone)]
// #[serde(untagged)]
pub struct ReplyParameters {
    /// message_id	Integer	Identifier of the message that will be replied to in the current chat,
    /// or in the chat chat_id if it is specified
    message_id: i64,
    /// chat_id	Integer or String	Optional.
    /// If the message to be replied to is from a different chat,
    /// unique identifier for the chat or username of the channel (in the format @channelusername). Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    chat_id: Option<i64>,
    /// allow_sending_without_reply	Boolean	Optional.
    /// Pass True if the message should be sent even if the specified message to be replied to is not found. Always False for replies in another chat or forum topic. Always True for messages sent on behalf of a business account.
    allow_sending_without_reply: Option<bool>,
    /// quote	String	Optional. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, and custom_emoji entities. The message will fail to send if the quote isn't found in the original message.
    quote: Option<String>,
    ///quote_parse_mode	String	Optional.
    /// Mode for parsing entities in the quote. See formatting options for more details.
    quote_parse_mode: Option<String>,
    ///quote_entities	Array of MessageEntity	Optional.
    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of quote_parse_mode.
    quote_entities: Option<Vec<MessageEntity>>,
    ///quote_position	Integer	Optional.
    /// Position of the quote in the original message in UTF-16 code units
    quote_position: Option<i64>,
    ///checklist_task_id	Integer	Optional.
    /// Identifier o
    checklist_task_id: Option<i64>,
}

impl ReplyParameters {
    pub fn new() -> Self {
        Self {
            message_id: 0,
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
            checklist_task_id: None,
        }
    }
    //todo
}
