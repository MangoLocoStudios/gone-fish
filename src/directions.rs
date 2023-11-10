#[derive(Clone, Copy)]
pub enum Directions {
    LEFT,
    RIGHT,
}

impl Directions {
    pub fn random() -> Self {
        match rand::random::<bool>() {
            true => Self::LEFT,
            false => Self::RIGHT,
        }
    }

    pub fn going_left(self) -> bool {
        match self {
            Self::LEFT => true,
            Self::RIGHT => false,
        }
    }
}
