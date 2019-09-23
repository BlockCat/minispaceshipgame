use amethyst::ecs::{Component, NullStorage, DenseVecStorage};

#[derive(Default)]
pub struct PlayerTag(pub bool);

#[derive(Default)]
pub struct AITag;

impl Component for PlayerTag {
    type Storage = DenseVecStorage<Self>;
}

impl Component for AITag {
    type Storage = NullStorage<Self>;
}
