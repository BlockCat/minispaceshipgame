use amethyst::ecs::{Component, DenseVecStorage};

pub struct Ship {
    pub id: i32,
    pub rotation: f32,
    pub x: f32,
    pub y: f32,
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

impl Ship {

    pub fn translate_x(&mut self, dx: f32) {
        self.x += dx;
    }
    pub fn translate_y(&mut self, dy: f32) {
        self.y += dy;
    }
    pub fn translate_xy(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn set_xy(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
}