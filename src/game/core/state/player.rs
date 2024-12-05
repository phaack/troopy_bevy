use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player(Option<u8>);

impl Player {
    pub fn new(player: Option<u8>) -> Self {
        Player(player)
    }

    pub fn default() -> Self {
        Player(None)
    }

    pub fn id(&self) -> Option<u8> {
        self.0
    }
    pub fn set_id(&mut self, id: u8) {
        self.0 = Some(id)
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
