#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ClientId, MessageRecipient};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum MessagePayload {
    Bytes(Vec<u8>),
    Text(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum RoomEvent {
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
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MessageFromRoom {
    Message {
        recipient: MessageRecipient,
        message: MessagePayload,
    },
}
