#![doc = include_str!("../README.md")]
use serde_derive::{Deserialize, Serialize};



#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
/// The side of the vehicle an attribute or signal is related to.
/// This type is used as a key inside topics.
pub enum Side {
    Left = 1,
    Right = 2,
}

impl Default for Side {
    fn default() -> Self {
        Side::Right
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
/// Front or Rear of the vehicle
/// This type is used as a key inside topics. 
pub enum Position {
    Front = 1,
    Rear = 2,
}

impl Default for Position {
    fn default() -> Self {
        Position::Front
    }
}

/// A global IEEE 1588/802.1AS timestamp is 80 bits in total, divided into two parts
#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Timestamp {
    ///seconds since epoch
    pub sec : u64,
    /// nanoseconds
    pub nsec : u32,
}

impl Default for Timestamp {
    fn default() -> Self {
        /* TODO: Should get system time and put it here*/
        //let now = std::time::Instant::now();
        Self {
            sec : 0,
            nsec : 0,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
