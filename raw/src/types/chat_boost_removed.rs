#[allow(unused_imports)]
use crate::{Chat, Document};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i32,
    // pub source:ChatBoostSource,
    pub source: bson::Document,
}

impl PartialOrd for ChatBoostRemoved {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.boost_id.cmp(&other.boost_id))
    }

    fn lt(&self, other: &Self) -> bool {
        self.boost_id.cmp(&other.boost_id).is_lt()
    }

    fn le(&self, other: &Self) -> bool {
        self.boost_id.cmp(&other.boost_id).is_le()
    }

    fn gt(&self, other: &Self) -> bool {
        self.boost_id.cmp(&other.boost_id).is_gt()
    }

    fn ge(&self, other: &Self) -> bool {
        self.boost_id.cmp(&other.boost_id).is_ge()
    }
}
