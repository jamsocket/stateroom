use crate::ClientId;
#[cfg(feature = "serde")]
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
    EveryoneExcept(ClientId),
}

impl MessageRecipient {
    #[must_use]
    pub fn encode_i32(&self) -> i32 {
        match self {
            Self::Broadcast => 0,
            Self::Client(c) => c.0 as i32,
            Self::EveryoneExcept(c) => -(c.0 as i32),
        }
    }

    #[must_use]
    pub fn decode_i32(enc_client_id: i32) -> Self {
        match enc_client_id {
            0 => Self::Broadcast,
            c if c > 0 => Self::Client((c as u32).into()),
            c => Self::EveryoneExcept((-c as u32).into()),
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
        assert_eq!(MessageRecipient::Broadcast, MessageRecipient::decode_i32(0));
        assert_eq!(
            MessageRecipient::Client(3.into()),
            MessageRecipient::decode_i32(3)
        );
        assert_eq!(
            MessageRecipient::Client(9.into()),
            ClientId::from(9u32).into()
        );

        assert_eq!(0, MessageRecipient::Broadcast.encode_i32());
        assert_eq!(443, MessageRecipient::Client(443.into()).encode_i32());

        assert_eq!(-4, MessageRecipient::EveryoneExcept(4.into()).encode_i32());
        assert_eq!(MessageRecipient::EveryoneExcept(119.into()), MessageRecipient::decode_i32(-119));

        assert_eq!(MessageRecipient::EveryoneExcept(1.into()), MessageRecipient::decode_i32(-1));
    }
}
