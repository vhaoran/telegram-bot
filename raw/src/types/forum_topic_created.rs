/*
"forum_topic_created":{"name":"dddddd","icon_color":9367192},"is_topic_messag
e":true}
*/

/*
name	String	Name of the topic
icon_color	Integer	Color of the topic icon in RGB format
icon_custom_emoji_id	String	Optional. Unique identifier of the custom emoj
*/
use crate::Integer;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: Integer,
    pub icon_custom_emoji_id: Option<String>,
}
