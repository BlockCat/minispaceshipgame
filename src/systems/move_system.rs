use amethyst::ecs::System;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{ WriteStorage, Read, ReadStorage, Join};
use amethyst::core::Transform;


use crate::components::Velocity;

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>              
    );

    fn run(&mut self, (mut transforms, velocity): Self::SystemData) {
        
        for (transform, vel) in (&mut transforms, &velocity).join() {
            transform.set_rotation_2d(vel.rotation);
            transform.move_up(vel.speed);

            println!("Entity: ({}, {})", transform.translation().x, transform.translation().y);
            

            if transform.translation().x < 0f32 {
                transform.prepend_translation_x(100.0);
            }
            if transform.translation().x > 100f32 {
                transform.prepend_translation_x(-100.0);
            }
            if transform.translation().y < 0f32 {
                transform.prepend_translation_y(100.0);
            }
            if transform.translation().y > 100f32 {
                transform.prepend_translation_y(-100.0);
            }
        }
    }
}