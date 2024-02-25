use crate::types::*;
#[allow(unused_imports)]
use crate::Chat;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostUpdate {
    pub chat: bson::Document,
    pub boost: bson::Document,
}
impl PartialOrd for ChatBoostUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b) = (self.chat.to_string(), other.chat.to_string());
        Some(a.cmp(&b))
    }

    fn lt(&self, other: &Self) -> bool {
        let (a, b) = (self.chat.to_string(), other.chat.to_string());
        a.cmp(&b).is_lt()
    }

    fn le(&self, other: &Self) -> bool {
        let (a, b) = (self.chat.to_string(), other.chat.to_string());
        a.cmp(&b).is_le()
    }

    fn gt(&self, other: &Self) -> bool {
        let (a, b) = (self.chat.to_string(), other.chat.to_string());
        a.cmp(&b).is_gt()
    }

    fn ge(&self, other: &Self) -> bool {
        let (a, b) = (self.chat.to_string(), other.chat.to_string());
        a.cmp(&b).is_ge()
    }
}

//-------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i32,
    pub expiration_date: i32,
    pub source: ChatBoostSource,
}
//-------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveway(ChatBoostSourceGiveaway),
}
//-------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatBoostSourcePremium {
    pub source: String,
    pub user: User,
}

//-------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}

//-------------------------------------
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    pub source: String,
    pub giveaway_message_id: i64,
    pub user: User,
    pub is_unclaimed: bool,
}

#[test]
fn aa() {
    //---------------------
    let s = r#"
{"update_id":156164228,
 "chat_boost":{
     "chat":{
         "id":-1001937088141,
         "title":"\u53e4\u7075\u652f\u4ed8\u5b9d\u652f\u4ed8\u6e20\u9053",
         "type":"supergroup"
      },
      "boost":{
         "boost_id":"6df0eb612d3062be970167d2ada6f837",
         "add_date":1708364528,
         "expiration_date":1709799036,
         "source":{
             "source":"premium",
             "user":{
                 "id":6323110545,
                 "is_bot":false,
                 "first_name":"\u6625\u8282\u4e0d\u653e\u5047\uff08\u5de5\u4f5c\u4e2d\uff09Q\u6296\u97f3\u5feb\u624b\u4e1a\u52a1\u8bf4\u660e\u5728\u7b80\u5386",
                 "last_name":"\u53e4\u7075\u7cbe\u602a\u4ee3\u5237\u6d41\u6c34",
                 "username":"GLJG888",
                 "language_code":"zh-hans",
                 "is_premium":true
             }
         }
      }
     }
 }    
 "#;
    let a: serde_json::Result<Update> = serde_json::from_str(s);
    println!("-----------{a:#?}-----------",);
}
