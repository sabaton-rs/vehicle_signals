use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
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
pub enum Position {
    Front = 1,
    Rear = 2,
}

impl Default for Position {
    fn default() -> Self {
        Position::Front
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
