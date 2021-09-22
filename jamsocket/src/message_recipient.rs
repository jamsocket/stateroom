use crate::ClientId;
#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

/// Represents the recipient(s) of a message.
///
/// Messages may either be sent to a particular client by numeric id
/// (`MessageRecipient::Client(3)`), or be broadcast to all connected clients
/// (`MessageRecipient::Broadcast`).]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub enum MessageRecipient {
    Broadcast,
    Client(ClientId),
}

impl MessageRecipient {
    #[must_use]
    pub fn encode_u32(&self) -> u32 {
        match self {
            Self::Broadcast => 0,
            Self::Client(c) => (*c).into(),
        }
    }

    #[must_use]
    pub fn decode_u32(enc_client_id: u32) -> Self {
        match enc_client_id {
            0 => Self::Broadcast,
            c => Self::Client(c.into()),
        }
    }
}

impl From<ClientId> for MessageRecipient {
    fn from(c: ClientId) -> Self {
        MessageRecipient::Client(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientId, MessageRecipient};

    #[test]
    fn test_decode() {
        assert_eq!(MessageRecipient::Broadcast, MessageRecipient::decode_u32(0));
        assert_eq!(
            MessageRecipient::Client(3.into()),
            MessageRecipient::decode_u32(3)
        );
        assert_eq!(
            MessageRecipient::Client(9.into()),
            ClientId::from(9u32).into()
        );

        assert_eq!(0, MessageRecipient::Broadcast.encode_u32());
        assert_eq!(443, MessageRecipient::Client(443.into()).encode_u32());
    }
}
