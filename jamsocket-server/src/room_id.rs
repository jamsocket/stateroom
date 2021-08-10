use rand::{thread_rng, Rng};
use serde::{Deserialize, de::Visitor};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use uuid::Uuid;

/// By the Hull-Dobell theorem, the period of a linear congruential generator
/// with c=1 will have a period of m iff the multiplier `a` satisfies:
/// - a - 1 is divisible by all prime factors of m
/// - a - 1 is divisible by 4 if m is divisible by 4
///
/// Our LCG is used to pick sequences of n > 3 letters, for which there are
/// 26^n possibilities. The prime factors of m are therefore 2 and 13, and
/// m is divisible by 4 for n>1. 52 is the lowest number that divides 2, 13,
/// and 4, so we use 52+1 = 53 as the multiplier.
const LCG_MULTIPLIER: u64 = 53;

const ASCII_A: u32 = 0x41;

const SHORTEST_SHORT_NAME: usize = 3;
const LONGEST_SHORT_NAME: usize = 13;

/// Trait for objects capable of generating a user ID.
pub trait RoomIdGenerator: Debug + Send + Sync {
    fn generate(&mut self) -> String;
}

#[derive(Debug)]
pub struct ShortRoomIdGenerator {
    state: u64,
    offset: u64,
    m: u64,
    length: usize,
}

impl RoomIdGenerator for ShortRoomIdGenerator {
    fn generate(&mut self) -> String {
        self.state = (LCG_MULTIPLIER * self.state + 1) % self.m;
        let mut val = (self.state + self.offset) % self.m;

        let mut chars: Vec<char> = Vec::with_capacity(self.length);

        for _ in 0..self.length {
            let c = (val % 26) as u32;
            chars.push(char::from_u32(c + ASCII_A).unwrap());
            val /= 26;
        }

        chars.iter().collect()
    }
}

impl ShortRoomIdGenerator {
    pub fn new(length: usize) -> Self {
        let mut rng = thread_rng();
        let m = 26u64.pow(length as u32);
        let offset = rng.gen_range(0..m);
        let seed_state = rng.gen_range(0..m);

        Self {
            state: seed_state,
            offset,
            m,
            length,
        }
    }
}

#[derive(Debug)]
pub struct ShortRoomIdGeneratorFactory(pub usize);

impl ShortRoomIdGeneratorFactory {
    pub fn new(length: usize) -> Self {
        ShortRoomIdGeneratorFactory(length)
    }
}

impl RoomIdGeneratorFactory for ShortRoomIdGeneratorFactory {
    fn build(&self) -> Box<dyn RoomIdGenerator> {
        let generator = ShortRoomIdGenerator::new(self.0);
        Box::new(generator)
    }
}

#[derive(Debug)]
pub struct UuidRoomIdGeneratorFactory;

impl RoomIdGeneratorFactory for UuidRoomIdGeneratorFactory {
    fn build(&self) -> Box<dyn RoomIdGenerator> {
        Box::new(UuidRoomIdGenerator)
    }
}

/// Assigns a user ID from a UUID.
#[derive(Debug)]
pub struct UuidRoomIdGenerator;

pub trait RoomIdGeneratorFactory: Debug {
    fn build(&self) -> Box<dyn RoomIdGenerator>;
}

impl RoomIdGenerator for UuidRoomIdGenerator {
    fn generate(&mut self) -> String {
        let my_uuid = Uuid::new_v4();
        my_uuid.to_string()
    }
}

/// Determines how new rooms are assigned an id.
#[derive(Debug)]
pub enum RoomIdStrategy {
    /// Rooms are created when they are first accessed.
    Implicit,
    /// Rooms are created with an explicit API call that provides an ID.
    Explicit,
    /// Room IDs are created by an endpoint, which returns an ID
    /// generated by the provided generator.
    Generator(Box<dyn RoomIdGeneratorFactory + Send + Sync>),
}

struct RoomIdStrategyVisitor;

impl<'de> Visitor<'de> for RoomIdStrategyVisitor {
    type Value = RoomIdStrategy;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        RoomIdStrategy::from_str(v).map_err(|_| serde::de::Error::custom("Could not parse RoomIdStrategy."))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A string like {short, uuid, api, implicit}.")
    }
}

impl<'de> Deserialize<'de> for RoomIdStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(RoomIdStrategyVisitor)
    }
}

impl RoomIdStrategy {
    pub fn explicit_room_creation_allowed(&self) -> bool {
        match self {
            RoomIdStrategy::Explicit => true,
            RoomIdStrategy::Implicit => true,
            RoomIdStrategy::Generator(_) => false,
        }
    }

    pub fn implicit_room_creation_allowed(&self) -> bool {
        match self {
            RoomIdStrategy::Explicit => false,
            RoomIdStrategy::Implicit => true,
            RoomIdStrategy::Generator(_) => false,
        }
    }

    pub fn try_generator(&self) -> Option<Box<dyn RoomIdGenerator>> {
        match self {
            RoomIdStrategy::Explicit => None,
            RoomIdStrategy::Implicit => Some(UuidRoomIdGeneratorFactory.build()),
            RoomIdStrategy::Generator(factory) => Some(factory.build()),
        }
    }
}

impl Default for RoomIdStrategy {
    fn default() -> Self {
        RoomIdStrategy::Implicit
    }
}

#[derive(Debug)]
pub struct BadGeneratorName(String);

impl Display for BadGeneratorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bad room ID generator '{}', expected one of {{singleton,short,uuid,api,implicit}}.",
            self.0
        )
    }
}

impl std::error::Error for BadGeneratorName {}

impl FromStr for RoomIdStrategy {
    type Err = BadGeneratorName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "api" => Ok(RoomIdStrategy::Explicit),
            "implicit" => Ok(RoomIdStrategy::Implicit),
            "short" => Ok(RoomIdStrategy::Generator(Box::new(
                ShortRoomIdGeneratorFactory(4),
            ))),
            "uuid" => Ok(RoomIdStrategy::Generator(Box::new(
                UuidRoomIdGeneratorFactory,
            ))),
            _ if s.starts_with("short") => {
                if let Some(num) = s.strip_prefix("short") {
                    let n: usize = num.parse().map_err(|_| BadGeneratorName(s.to_string()))?;

                    if !(SHORTEST_SHORT_NAME..LONGEST_SHORT_NAME).contains(&n) {
                        return Err(BadGeneratorName(s.to_string()));
                    }

                    Ok(RoomIdStrategy::Generator(Box::new(
                        ShortRoomIdGeneratorFactory(n),
                    )))
                } else {
                    panic!() // Should never get here.
                }
            }
            _ => Err(BadGeneratorName(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn test_parse_room_id_strategy() {
        assert_matches!(
            RoomIdStrategy::from_str("api").unwrap(),
            RoomIdStrategy::Explicit
        );
        assert_matches!(
            RoomIdStrategy::from_str("implicit").unwrap(),
            RoomIdStrategy::Implicit
        );
        assert_matches!(
            RoomIdStrategy::from_str("short").unwrap(),
            RoomIdStrategy::Generator(_)
        );
        assert_matches!(
            RoomIdStrategy::from_str("short5").unwrap(),
            RoomIdStrategy::Generator(_)
        );
        assert_matches!(
            RoomIdStrategy::from_str("uuid").unwrap(),
            RoomIdStrategy::Generator(_)
        );
    }

    #[test]
    fn test_length() {
        if let RoomIdStrategy::Generator(g) = RoomIdStrategy::from_str("uuid").unwrap() {
            let result = g.build().generate();
            assert_eq!(36, result.len())
        } else {
            panic!("Expected RoomIdStrategy::Generator.")
        }

        if let RoomIdStrategy::Generator(g) = RoomIdStrategy::from_str("short6").unwrap() {
            let result = g.build().generate();
            assert_eq!(6, result.len())
        } else {
            panic!("Expected RoomIdStrategy::Generator.")
        }

        if let RoomIdStrategy::Generator(g) = RoomIdStrategy::from_str("short").unwrap() {
            let result = g.build().generate();
            assert_eq!(4, result.len())
        } else {
            panic!("Expected RoomIdStrategy::Generator.")
        }
    }
}
