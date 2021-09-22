#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct ClientId(u32);

impl From<ClientId> for u32 {
    fn from(c: ClientId) -> Self {
        c.0
    }
}

impl From<u32> for ClientId {
    fn from(u: u32) -> Self {
        ClientId(u)
    }
}
