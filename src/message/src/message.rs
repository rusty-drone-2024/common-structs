use crate::{FileWithData, Link, Media, ServerType};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;
use wg_2024::packet::{Fragment, FRAGMENT_DSIZE};

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

impl Message {
    pub fn into_fragments(self) -> Vec<Fragment> {
        let mut bytes = serde_json::to_vec(&self).unwrap();
        let len_multiple = bytes.len().div_ceil(FRAGMENT_DSIZE) * FRAGMENT_DSIZE;
        bytes.resize(len_multiple, 0);

        let chunks = bytes.chunks(FRAGMENT_DSIZE);
        let len = chunks.len();
        let mut res = vec![];

        chunks.for_each(|chunk| {
            let fixed_chunk = <[u8; 128]>::try_from(chunk).unwrap();
            res.push(Fragment::new(res.len() as u64, len as u64, fixed_chunk));
        });

        res
    }

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
