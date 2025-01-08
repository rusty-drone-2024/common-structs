mod message;

pub use message::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wg_2024::network::NodeId;

pub type Link = String;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum ServerType {
    Chat,
    Text,
    Media,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct FileWithData {
    file: Vec<char>,
    // For each link which server to get the data from
    // Data is both file and media
    related_data: HashMap<Link, NodeId>,
}
