use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct PlayerTag;

#[derive(Default)]
pub struct AITag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}

impl Component for AITag {
    type Storage = NullStorage<Self>;
}
