mod message;

pub use message::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wg_2024::network::NodeId;

pub type Link = String;
pub type Media = Vec<u8>;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileWithData {
    pub file: String,
    // For each link which server to get the data from
    // Data is both file and media
    pub related_data: HashMap<Link, NodeId>,
}
