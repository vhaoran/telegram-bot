use crate::Integer;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, };

use crate::types::{Chat, ChatInviteLink, User};

/// Represents a join request sent to a chat.
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    // #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: Integer,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

#[test]
fn a_1() {
    //---------------------
    let s = r#"
{
    "chat": {"id":-1001355255437,
                    "title":"abc",
                    "username":"HLSXZWP",
                    "is_forum":true,
                    "type":"supergroup"
    },
    "from":{"id":5216941809,
        "is_bot":false,
        "first_name":"ffff_name",
        "username":"uname",
        "language_code":"zh-hans"
    },
    "date":1670159122
}
"#;

    let r = serde_json::from_str::<ChatJoinRequest>(s);
    println!("-----------{r:#?}-----------",);
}
