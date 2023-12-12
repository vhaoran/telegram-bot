/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputFileX {
    /// FileID is an ID of a file already uploaded to Telegram.
    FileID(String),
    /// FileURL is a URL to use as a file for a request.
    FileURL(String),
    /// fileAttach is an internal file type used for processed media groups.
    FileAttach(String),
    /// FileBytes contains information about a set of bytes to upload as a File.
    FileBytes(String, Vec<u8>),
    /// FilePath is a path to a local file.
    FilePath(String),
}

/// On success,returns a InputFileXResult object data method
pub enum InputFileXResult {
    /// don't need upload
    Text(String),
    /// must upload using multipart/form-data
    Part(reqwest::multipart::Part),
}

impl InputFileX {
    pub fn need_upload(&self) -> bool {
        matches!(self, InputFileX::FileBytes(_, _) | InputFileX::FilePath(_))
    }

    pub async fn data(&self) -> Result<InputFileXResult, Box<dyn std::error::Error>> {
        match self {
            InputFileX::FileID(id) => Ok(InputFileXResult::Text(id.clone())),
            InputFileX::FileURL(url) => Ok(InputFileXResult::Text(url.clone())),
            InputFileX::FileAttach(attach) => Ok(InputFileXResult::Text(attach.clone())),
            InputFileX::FileBytes(file_name, bytes) => Ok(InputFileXResult::Part(
                reqwest::multipart::Part::bytes(bytes.clone()).file_name(file_name.to_string()),
            )),
            InputFileX::FilePath(path) => Ok(InputFileXResult::Part(
                reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(
                    tokio_util::codec::FramedRead::new(
                        tokio::fs::File::open(path).await?,
                        tokio_util::codec::BytesCodec::new(),
                    ),
                ))
                    .file_name(path.to_string()),
            )),
        }
    }
}
