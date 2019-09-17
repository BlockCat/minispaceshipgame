use amethyst::ecs::System;
use amethyst::ecs::{ WriteStorage, ReadStorage, Join};
use amethyst::core::Transform;


use crate::components::Velocity;
use crate::components::Ship;


pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Ship>,
        ReadStorage<'a, Velocity>              
    );

    fn run(&mut self, (mut transforms, mut ships, velocity): Self::SystemData) {
        
        for (transform, ship, vel) in (&mut transforms, &mut ships, &velocity).join() {
            ship.translate_xy(vel.dx, vel.dy);            

            if ship.x < 0f32 {
                ship.translate_x(crate::ARENA_WIDTH);                
            }
            if ship.x > crate::ARENA_WIDTH {
                ship.translate_x(-crate::ARENA_WIDTH);
            }
            if ship.y < 0f32 {
                ship.translate_y(crate::ARENA_HEIGHT);
            }
            if ship.y > crate::ARENA_HEIGHT {
                ship.translate_y(-crate::ARENA_HEIGHT);
            }            

            transform.set_rotation_2d(0.0);
            transform.set_translation_x(ship.x);
            transform.set_translation_y(ship.y);
            transform.set_rotation_2d(ship.rotation + std::f32::consts::FRAC_PI_2);
        }
    }
}