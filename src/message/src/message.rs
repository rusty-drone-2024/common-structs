use std::fmt::{Display, Formatter};
use crate::{FileWithData, Link, Media, ServerType};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;
use crate::Message::*;

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
            ReqChatSend { to, .. } => &format!("ReqChatSend({to}, _)"),
            RespFile(_) => "RespFile(_)",
            RespMedia(_) => "RespMedia(_)",
            RespChatFrom { from, .. } => &format!("RespChatFrom({from}, _)"),
            other => &format!("{:?}", other),
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn basic_tests() {
        let message = Message::ReqServerType;
        let message2 = Message::from_fragments(message.clone().into_fragments());
        assert_eq!(Ok(message), message2);
    }
}
