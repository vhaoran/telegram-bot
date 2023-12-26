use crate::{InputFile, MessageEntity};

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputMedia {
    // #[serde(rename = "animation")]
    // InputMediaAnimation(InputMediaAnimation),
    #[serde(rename = "document")]
    InputMediaDocument(InputMediaDocument),
    #[serde(rename = "audio")]
    InputMediaAudio(InputMediaAudio),
    #[serde(rename = "photo")]
    InputMediaPhoto(InputMediaPhoto),
    #[serde(rename = "video")]
    InputMediaVideo(InputMediaVideo),
}

/// Represents a photo to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
}
impl InputMediaPhoto {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }

    pub fn caption(&mut self, caption: String) -> &mut Self {
        self.caption = Some(caption);
        self
    }

    pub fn parse_mode(&mut self, parse_mode: String) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn caption_entities(&mut self, caption_entities: Vec<MessageEntity>) -> &mut Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn caption_entities_opt(&mut self, caption_entities: Option<Vec<MessageEntity>>) -> &mut Self {
        self.caption_entities = caption_entities;
        self
    }

}

/// Represents a video to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }
    pub fn caption(&mut self, caption: String) -> &mut Self {
        self.caption = Some(caption);
        self
    }
    pub fn thumb(&mut self, thumb: InputFile) -> &mut Self {
        self.thumb = Some(thumb);
        self
    }
    pub fn parse_mode(&mut self, parse_mode: String) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn caption_entities(&mut self, caption_entities: Vec<MessageEntity>) -> &mut Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn caption_entities_opt(&mut self, caption_entities: Option<Vec<MessageEntity>>) -> &mut Self {
        self.caption_entities = caption_entities;
        self
    }
    pub fn width(&mut self, width: i64) -> &mut Self {
        self.width = Some(width);
        self
    }
    pub fn height(&mut self, height: i64) -> &mut Self {
        self.height = Some(height);
        self
    }
    pub fn duration(&mut self, duration: i64) -> &mut Self {
        self.duration = Some(duration);
        self
    }
    pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAnimation {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the animation to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Animation duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
impl InputMediaAnimation {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
        }
    }
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAudio {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl InputMediaAudio {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    pub fn caption(&mut self, caption: String) -> &mut Self {
        self.caption = Some(caption);
        self
    }
    pub fn caption_entities(&mut self, caption_entities: Vec<MessageEntity>) -> &mut Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn caption_entities_opt(&mut self, caption_entities: Option<Vec<MessageEntity>>) -> &mut Self {
        self.caption_entities = caption_entities;
        self
    }

    pub fn thumb(&mut self, thumb: InputFile) -> &mut Self {
        self.thumb = Some(thumb);
        self
    }
    pub fn parse_mode(&mut self, parse_mode: String) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn performer(&mut self, performer: String) -> &mut Self {
        self.performer = Some(performer);
        self
    }
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn duration(&mut self, duration: i64) -> &mut Self {
        self.duration = Some(duration);
        self
    }
}

/// Represents a general file to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaDocument {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always True, if the document is sent as part of an album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}
impl InputMediaDocument {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }

    pub fn caption(&mut self, caption: String) -> &mut Self {
        self.caption = Some(caption);
        self
    }
    pub fn caption_entities(&mut self, caption_entities: Vec<MessageEntity>) -> &mut Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn caption_entities_opt(&mut self, caption_entities: Option<Vec<MessageEntity>>) -> &mut Self {
        self.caption_entities = caption_entities;
        self
    }

    pub fn thumb(&mut self, thumb: InputFile) -> &mut Self {
        self.thumb = Some(thumb);
        self
    }
    pub fn parse_mode(&mut self, parse_mode: String) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn disable_content_type_detection(
        &mut self,
        disable_content_type_detection: bool,
    ) -> &mut Self {
        self.disable_content_type_detection = Some(disable_content_type_detection);
        self
    }


}
