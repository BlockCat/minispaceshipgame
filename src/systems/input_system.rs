use amethyst::ecs::System;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{ WriteStorage, Read, ReadStorage, Join};


use crate::components::PlayerTag;
use crate::components::Velocity;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, PlayerTag>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, _, input): Self::SystemData) {
        let movement = input.axis_value("speed");
        for (mut transform) in (&mut transforms).join() {
            if let Some(mv) = movement {
                println!("Speed {}", mv);
            }
        }
    }
}