use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity {    
    pub dx: f32,
    pub dy: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {

    pub fn lenght(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }
    pub fn normalize(&mut self) {
        let dist = 1.0 / self.lenght();
        self.dx *= dist;
        self.dy *= dist;
    }
}