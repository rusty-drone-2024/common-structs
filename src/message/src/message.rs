use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;
use wg_2024::packet::{Fragment, FRAGMENT_DSIZE};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
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
    ErrUnsupporedRequestType,
    // S -> C file
    RespFilesList(Vec<u64>),
    RespFile(FileWithMedia),
    // S -> C media
    RespMedia(Vec<u8>),
    ErrNotFound,
    // S -> C client
    RespClientList(Vec<NodeId>),
    RespChatFrom { from: NodeId, chat_msg: Vec<u8> },
    ErrNotExistentClient,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum ServerType {
    Chat,
    Text,
    Media,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct FileWithMedia {
    file: Vec<u8>,
    related_media_ids: Vec<u64>,
    related_media_server_id: NodeId,
}

impl Message {
    pub fn into_fragment(self) -> Vec<Fragment> {
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

    pub fn from_fragments_hashmap(mut fragments: Vec<Fragment>) -> Option<Self> {
        let mut bytes = vec![];
        fragments.sort_by(|a, b| a.fragment_index.cmp(&b.fragment_index));

        for (i, el) in fragments.iter().enumerate() {
            if i as u64 != el.fragment_index {
                return None;
            }

            bytes.extend_from_slice(&el.data);
        }

        serde_json::from_slice(&bytes).ok()
    }
}
