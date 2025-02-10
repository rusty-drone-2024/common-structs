use crate::Message;
use wg_2024::packet::{Fragment, FRAGMENT_DSIZE};

impl Message {
    /// Convert a message into (multiple) fragments to be able to send it over the network.
    /// These fragments should be wrapped in MsgFragments packets.
    pub fn into_fragments(self) -> Vec<Fragment> {
        // Convert to bytes
        let mut bytes = serde_json::to_vec(&self).unwrap();

        // Split into chunks of bytes that fit into fragments
        let len_multiple = bytes.len().div_ceil(FRAGMENT_DSIZE) * FRAGMENT_DSIZE;
        bytes.resize(len_multiple, 0);
        let chunks = bytes.chunks(FRAGMENT_DSIZE);

        // Wrap chunks into fragments
        let mut res = vec![];
        let len = chunks.len();
        chunks.for_each(|chunk| {
            let fixed_chunk = <[u8; 128]>::try_from(chunk).unwrap();
            res.push(Fragment::new(res.len() as u64, len as u64, fixed_chunk));
        });
        res
    }

    /// Convert fragments back into its message form.
    /// Fragments will be sorted by fragment index.
    pub fn from_fragments(mut fragments: Vec<Fragment>) -> Result<Self, String> {
        let mut bytes = vec![];

        // Sort fragments by fragment index.
        fragments.sort_by(|a, b| a.fragment_index.cmp(&b.fragment_index));

        // Concatenate the bytes stored in each fragment.
        for (i, el) in fragments.iter().enumerate() {
            if i as u64 != el.fragment_index {
                return Err("fragment is missing".to_string());
            }

            bytes.extend_from_slice(&el.data[0..(el.length as usize)]);
        }

        // Parse the bytes back into a message
        serde_json::from_slice(&bytes)
            .ok()
            .ok_or("Content is not of correct type".to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    /// Converting a message into fragments and back yields the same message.
    fn basic_tests() {
        let message = Message::ReqServerType;
        let message2 = Message::from_fragments(message.clone().into_fragments());
        assert_eq!(Ok(message), message2);
    }
}
