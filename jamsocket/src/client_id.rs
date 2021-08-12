#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
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
