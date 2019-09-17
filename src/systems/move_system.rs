use amethyst::ecs::System;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{ WriteStorage, Read, ReadStorage, Join};
use amethyst::core::Transform;

use crate::components::PlayerTag;
use crate::components::Velocity;

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, PlayerTag>,        
    );

    fn run(&mut self, (mut transforms, velocity, _): Self::SystemData) {
        
        for (transform, vel) in (&mut transforms, &velocity).join() {
            transform.prepend_translation_x(vel.dx);
            transform.prepend_translation_y(vel.dy);
        }
    }
}