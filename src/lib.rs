use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Side::Right
    }
}

#[derive(Serialize, Deserialize)]
pub enum Position {
    Front,
    Rear,
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
