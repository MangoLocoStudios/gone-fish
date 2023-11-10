use bevy::prelude::{Component, Mut};

//TODO: Track what kinds of fish are "in" the storage (for the port)
#[derive(Component)]
pub struct FishStorage {
    pub current: u32,
    pub max: u32,
}

impl FishStorage {
    pub fn update_storage (new_current: u32,
                           new_max: Option<u32>,
                           storage: &mut Mut<Self>)
    {
        storage.current = new_current;

        if let Some(new_max) = new_max {
            storage.max = new_max
        }
    }
}
