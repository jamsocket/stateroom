#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ClientId, MessageRecipient};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MessagePayload {
    Bytes(Vec<u8>),
    Text(String),
}

impl MessagePayload {
    pub fn text(&self) -> Option<&str> {
        match self {
            MessagePayload::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        match self {
            MessagePayload::Bytes(b) => Some(b),
            _ => None,
        }
    }
}

impl From<String> for MessagePayload {
    fn from(val: String) -> Self {
        MessagePayload::Text(val)
    }
}

impl From<&str> for MessagePayload {
    fn from(val: &str) -> Self {
        MessagePayload::Text(val.to_string())
    }
}

impl From<Vec<u8>> for MessagePayload {
    fn from(val: Vec<u8>) -> Self {
        MessagePayload::Bytes(val)
    }
}

impl From<&[u8]> for MessagePayload {
    fn from(val: &[u8]) -> Self {
        MessagePayload::Bytes(val.to_vec())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MessageToProcess {
    Init,
    Connect {
        client: ClientId,
    },
    Disconnect {
        client: ClientId,
    },
    Message {
        sender: ClientId,
        message: MessagePayload,
    },
    Timer,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MessageFromProcess {
    Message {
        recipient: MessageRecipient,
        message: MessagePayload,
    },
    SetTimer {
        ms_delay: u32,
    },
}
