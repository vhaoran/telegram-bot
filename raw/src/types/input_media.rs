use crate::{InputFileX, MessageEntity};

#[derive(Deserialize, Serialize, Debug, Clone)]
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

impl InputMedia {
    /// prepare_input_media_param evaluates a single InputMedia and determines if it
    /// needs to be modified for a successful upload. If it returns nil, then the
    /// value does not need to be included in the params. Otherwise, it will return
    /// the same type as was originally provided.
    ///
    /// The idx is used to calculate the file field name. If you only have a single
    /// file, 0 may be used. It is formatted into "attach://file-%d" for the primary
    /// media and "attach://file-%d-thumb" for thumbnails.
    ///
    /// It is expected to be used in conjunction with prepareInputMediaFile.
    pub fn prepare_input_media_param(&self, idx: i32) -> Self {
        match self {
            // InputMedia::InputMediaAnimation(animation) => {
            //     let mut media = animation.media.clone();
            //     // if media.need_upload() {
            //     //     media = Self::attach_file(idx);
            //     // }
            //     let mut thumb: Option<InputFileX> = None;
            //     if let Some(some_thumb) = &animation.thumb {
            //         // if some_thumb.need_upload() {
            //         //     thumb = Some(Self::attach_thumb_file(idx));
            //         // }
            //     }
            //     Self::InputMediaAnimation(InputMediaAnimation {
            //         media,
            //         thumb,
            //         ..animation.clone()
            //     })
            // }
            InputMedia::InputMediaDocument(document) => {
                let mut media = document.media.clone();
                // if media.need_upload() {
                //     media = Self::attach_file(idx);
                // }
                let mut thumb: Option<InputFileX> = None;
                if let Some(some_thumb) = &document.thumb {
                    // if some_thumb.need_upload() {
                    //     thumb = Some(Self::attach_thumb_file(idx));
                    // }
                }
                Self::InputMediaDocument(InputMediaDocument {
                    media,
                    thumb,
                    ..document.clone()
                })
            }
            InputMedia::InputMediaAudio(audio) => {
                let mut media = audio.media.clone();
                // if media.need_upload() {
                //     media = Self::attach_file(idx);
                // }
                let mut thumb: Option<InputFileX> = None;
                if let Some(some_thumb) = &audio.thumb {
                    // if some_thumb.need_upload() {
                    //     thumb = Some(Self::attach_thumb_file(idx));
                    // }
                }
                Self::InputMediaAudio(InputMediaAudio {
                    media,
                    thumb,
                    ..audio.clone()
                })
            }
            InputMedia::InputMediaPhoto(photo) => {
                // if !photo.media.need_upload() {
                //     return self.clone();
                // }
                Self::InputMediaPhoto(InputMediaPhoto {
                    media: Self::attach_file(idx),
                    ..photo.clone()
                })
            }
            InputMedia::InputMediaVideo(video) => {
                let mut media = video.media.clone();
                // if media.need_upload() {
                //     media = Self::attach_file(idx);
                // }
                let mut thumb: Option<InputFileX> = None;
                if let Some(some_thumb) = &video.thumb {
                    // if some_thumb.need_upload() {
                    //     thumb = Some(Self::attach_thumb_file(idx));
                    // }
                }
                Self::InputMediaVideo(InputMediaVideo {
                    media,
                    thumb,
                    ..video.clone()
                })
            }
        }
    }

    /// prepare_input_media_file generates an array of RequestFile to provide for
    /// Fileable's files method. It returns an array as a single InputMedia may have
    /// multiple files, for the primary media and a thumbnail.
    ///
    /// The idx parameter is used to generate file field names. It uses the names
    /// "file-%d" for the main file and "file-%d-thumb" for the thumbnail.
    ///
    /// It is expected to be used in conjunction with prepareInputMediaParam.
    pub fn prepare_input_media_file(&self, idx: i32) -> Vec<(String, InputFileX)> {
        let mut result: Vec<(String, InputFileX)> = Vec::new();
        match self {
            // InputMedia::InputMediaAnimation(animation) => {
            //     let media = animation.media.clone();
            //     // if media.need_upload() {
            //     //     result.push((Self::attach_file_name(idx), media.clone()));
            //     // }
            //     if let Some(thumb) = &animation.thumb {
            //         // if thumb.need_upload() {
            //         //     result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
            //         // }
            //     }
            // }
            InputMedia::InputMediaDocument(document) => {
                let media = document.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &document.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
            InputMedia::InputMediaAudio(audio) => {
                let media = audio.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &audio.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
            InputMedia::InputMediaPhoto(photo) => {
                if photo.media.need_upload() {
                    result.push((Self::attach_file_name(idx), photo.media.clone()));
                }
            }
            InputMedia::InputMediaVideo(video) => {
                let media = video.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &video.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
        }
        result
    }

    fn attach_file_name(idx: i32) -> String {
        format!("file-{}", idx)
    }

    fn attach_thumb_file_name(idx: i32) -> String {
        format!("file-{}-thumb", idx)
    }

    fn attach_file(idx: i32) -> InputFileX {
        InputFileX::FileAttach(format!("attach://file-{}", idx))
    }

    fn attach_thumb_file(idx: i32) -> InputFileX {
        InputFileX::FileAttach(format!("attach://file-{}-thumb", idx))
    }
}

/// Represents a photo to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFileX,
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
    pub fn new(media: InputFileX) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }
}

/// Represents a video to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFileX,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileX>,
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
    pub fn new(media: InputFileX) -> Self {
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
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaAnimation {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFileX,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileX>,
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
    pub fn new(media: InputFileX) -> Self {
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaAudio {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFileX,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileX>,
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
    pub fn new(media: InputFileX) -> Self {
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
}

/// Represents a general file to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaDocument {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFileX,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileX>,
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
    pub fn new(media: InputFileX) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }
}
