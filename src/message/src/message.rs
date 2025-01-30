use crate::Message::*;
use crate::{FileWithData, Link, Media, ServerType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wg_2024::network::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    // C -> S
    ReqServerType,
    // C -> S file
    ReqFilesList,
    ReqFile(Link),
    // C -> S media
    ReqMedia(Link),
    // C -> S chat
    ReqChatRegistration,
    ReqChatClients,
    ReqChatSend { to: NodeId, chat_msg: Vec<u8> },

    // S-> C
    RespServerType(ServerType),
    ErrUnsupportedRequestType,
    // S -> C file
    RespFilesList(Vec<Link>),
    RespFile(FileWithData),
    // S -> C media
    RespMedia(Media),
    ErrNotFound,
    // S -> C client
    RespClientList(Vec<NodeId>),
    RespChatFrom { from: NodeId, chat_msg: Vec<u8> },
    ErrNotExistentClient,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RespFilesList(list) => &format!("RespFilesList(_ [size: {}])", list.len()),
            RespFile(_) => "RespFile(_)",
            RespMedia(_) => "RespMedia(_)",
            ReqChatSend { to, .. } => &format!("ReqChatSend({to}, _)"),
            RespChatFrom { from, .. } => &format!("RespChatFrom({from}, _)"),
            other => &format!("{:?}", other),
        };

        write!(f, "{}", str)
    }
}
