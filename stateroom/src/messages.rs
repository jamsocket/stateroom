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

impl Into<MessagePayload> for String {
    fn into(self) -> MessagePayload {
        MessagePayload::Text(self)
    }
}

impl Into<MessagePayload> for &str {
    fn into(self) -> MessagePayload {
        MessagePayload::Text(self.to_string())
    }
}

impl Into<MessagePayload> for Vec<u8> {
    fn into(self) -> MessagePayload {
        MessagePayload::Bytes(self)
    }
}

impl Into<MessagePayload> for &[u8] {
    fn into(self) -> MessagePayload {
        MessagePayload::Bytes(self.to_vec())
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
