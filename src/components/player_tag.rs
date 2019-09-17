use amethyst::ecs::{Component, NullStorage, FlaggedStorage};

#[derive(Default)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}
