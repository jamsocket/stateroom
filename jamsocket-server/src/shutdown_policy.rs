use serde::Deserialize;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct BadShutdownPolicyName(String);

impl Display for BadShutdownPolicyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bad shutdown policy '{}', expected one of {{never,immediate,<n>sec (for integer n > 0)}}.",
            self.0
        )
    }
}

impl std::error::Error for BadShutdownPolicyName {}

/// Determines how a server will behave when a room is empty (all clients have left).
///
/// Depending on this value, the server may shut down the room immediately, after a
/// given timeout, or never.
#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub enum ServiceShutdownPolicy {
    /// Never garbage collect rooms. This is mostly for development, or for when you
    /// have a fixed set of rooms.
    Never,

    /// Immediately delete rooms when they become empty.
    Immediate,

    /// Delete empty rooms if they stay empty for the given number of seconds. This
    /// gives clients a chance to reconnect without losing state.
    AfterSeconds(u32),
}

impl Default for ServiceShutdownPolicy {
    fn default() -> Self {
        ServiceShutdownPolicy::Immediate
    }
}

impl FromStr for ServiceShutdownPolicy {
    type Err = BadShutdownPolicyName;

    fn from_str(s: &str) -> Result<Self, BadShutdownPolicyName> {
        Ok(match s {
            "never" => ServiceShutdownPolicy::Never,
            "immediate" => ServiceShutdownPolicy::Immediate,
            _ if s.ends_with("sec") => {
                let v: u32 = s
                    .strip_suffix("sec")
                    .unwrap()
                    .parse()
                    .map_err(move |_| BadShutdownPolicyName(s.to_string()))?;
                ServiceShutdownPolicy::AfterSeconds(v)
            }
            _ => return Err(BadShutdownPolicyName(s.to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{shutdown_policy::BadShutdownPolicyName, ServiceShutdownPolicy};

    #[test]
    fn test_from_string() {
        assert_eq!(
            ServiceShutdownPolicy::Never,
            ServiceShutdownPolicy::from_str("never").unwrap()
        );
        assert_eq!(
            ServiceShutdownPolicy::Immediate,
            ServiceShutdownPolicy::from_str("immediate").unwrap()
        );
        assert_eq!(
            ServiceShutdownPolicy::AfterSeconds(30),
            ServiceShutdownPolicy::from_str("30sec").unwrap()
        );
        assert_eq!(
            ServiceShutdownPolicy::AfterSeconds(600),
            ServiceShutdownPolicy::from_str("600sec").unwrap()
        );

        assert_eq!(
            BadShutdownPolicyName("msec".to_string()),
            ServiceShutdownPolicy::from_str("msec").unwrap_err()
        );
        assert_eq!(
            BadShutdownPolicyName("blah".to_string()),
            ServiceShutdownPolicy::from_str("blah").unwrap_err()
        );
    }
}
