#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ClientId, MessageRecipient};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MessagePayload {
    Bytes(Vec<u8>),
    Text(String),
}

impl Into<MessagePayload> for String {
    fn into(self) -> MessagePayload {
        MessagePayload::Text(self.to_string())
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
