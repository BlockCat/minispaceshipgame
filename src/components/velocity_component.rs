use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct Velocity {
    pub speed: f32,
    pub rotation: f32    
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
