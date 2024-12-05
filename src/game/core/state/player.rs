pub type PlayerId = u8;

pub struct Player(Option<PlayerId>);

impl Player {
    pub fn new(player: Option<PlayerId>) -> Self {
        Player(player)
    }

    pub fn default() -> Self {
        Player(None)
    }

    pub fn id(&self) -> Option<PlayerId> {
        self.0
    }
    pub fn set_id(&mut self, id: PlayerId) {
        self.0 = Some(id)
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
