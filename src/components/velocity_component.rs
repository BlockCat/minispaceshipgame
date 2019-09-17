use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct Velocity {
    pub dx: f32,
    pub dy: f32    
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
