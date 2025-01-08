use crate::{FileWithData, Media, ServerType};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;
use wg_2024::packet::{Fragment, FRAGMENT_DSIZE};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    // C -> S
    ReqServerType,
    // C -> S file
    ReqFilesList,
    ReqFile(u64),
    // C -> S media
    ReqMedia(u64),
    // C -> S chat
    ReqChatRegistration,
    ReqChatClients,
    ReqChatSend { to: NodeId, chat_msg: Vec<u8> },

    // S-> C
    RespServerType(ServerType),
    ErrUnsupportedRequestType,
    // S -> C file
    RespFilesList(Vec<u64>),
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

    pub fn from_fragments(mut fragments: Vec<Fragment>) -> Result<Self, String> {
        let mut bytes = vec![];
        fragments.sort_by(|a, b| a.fragment_index.cmp(&b.fragment_index));

        for (i, el) in fragments.iter().enumerate() {
            if i as u64 != el.fragment_index {
                return Err("fragment is missing".to_string());
            }

            bytes.extend_from_slice(&el.data[0..(el.length as usize)]);
        }
        serde_json::from_slice(&bytes)
            .ok()
            .ok_or("Content is not of correct type".to_string())
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
