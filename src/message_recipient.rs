#[derive(Debug, Clone)]

/// Represents the recipient(s) of a message.
///
/// Messages may either be sent to a particular user by numeric id
/// (`MessageRecipient::User(3)`), or be broadcast to all connected users
/// (`MessageRecipient::Broadcast`).
pub enum MessageRecipient {
    Broadcast,
    User(u32),
}

impl MessageRecipient {
    pub fn encode_u32(&self) -> u32 {
        match self {
            Self::Broadcast => 0,
            Self::User(u) => *u,
        }
    }

    pub fn decode_u32(d: u32) -> Self {
        match d {
            0 => Self::Broadcast,
            u => Self::User(u),
        }
    }
}

impl From<u32> for MessageRecipient {
    fn from(u: u32) -> Self {
        MessageRecipient::User(u)
    }
}
