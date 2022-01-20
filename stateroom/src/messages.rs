#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ClientId, MessageRecipient};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MessagePayload {
    Bytes(Vec<u8>),
    Text(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MessageToProcess {
    Connect {
        client: ClientId,
    },
    Disconnect {
        client: ClientId,
    },
    Message {
        client: ClientId,
        message: MessagePayload,
    },
    Timer,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MessageFromProcess {
    Message {
        recipient: MessageRecipient,
        message: MessagePayload,
    },
}
