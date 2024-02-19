use crate::{Chat, Integer};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: Integer,
}