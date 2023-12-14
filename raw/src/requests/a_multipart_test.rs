use crate::requests::*;
use crate::types::*;
use crate::{multipart_map, ChatRef, InputFileX, InputMedia, Multipart, ToMultipart};
use std::collections::HashMap;

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
#[derive(Debug, Clone)]
pub struct XX {
    pub i64_id: i64,
    pub i64_list: Vec<i64>,
}

impl ToMultipart for XX {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (i64_id (text));
            (i64_list (json));
        }
    }
}

#[test]
fn audio_multi_test() -> anyhow::Result<()> {
    //---------------------
    let cid = 5_i64;
    let c = ChatRef::from_chat_id(ChatId::new(cid));
    // let mut a = SendAudio::new(c, InputFileRef::new("good"));
    let mut a = SendAudio::new(
        c,
        InputFileUpload::with_path("./abc").file_name("file_name.av"),
    );
    a.title("good".to_string());

    let r = a.to_multipart()?;
    println!("-----------{r:#?}-----------");
    Ok(())
}

#[test]
fn yyy() -> anyhow::Result<()> {
    //---------------------
    let src = XX {
        i64_id: 222,
        i64_list: vec![3, 4, 5],
    };

    let cid = 5_i64;
    let r = src.to_multipart()?;
    println!("-----------{r:#?}-----------",);
    Ok(())
}
