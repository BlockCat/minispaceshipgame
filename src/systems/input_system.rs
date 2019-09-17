use amethyst::ecs::System;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{ WriteStorage, Read, ReadStorage, Join};
use amethyst::core::transform::Transform;

use crate::components::{ PlayerTag, AITag};
use crate::components::Velocity;

pub struct InputSystem;
pub struct AISystem;

const TWO_PI: f32 = std::f32::consts::PI * 2.0;
const RANGE: f32 = 20.0 * 20.0;


impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, PlayerTag>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, tags, input): Self::SystemData) {
        let movement = input.axis_value("speed");
        let leftright = input.axis_value("leftright");
        for (velocity, _) in (&mut transforms, &tags).join() {
            if let Some(mv) = movement {
                if mv > 0.0 {
                    velocity.speed += 0.1;
                }
                if mv < 0.0 {
                    velocity.speed -= 0.3;
                }                
            }
            if let Some(lr) = leftright {
                velocity.rotation += lr * 0.1;
            }

            limit_velocity(velocity);           
            
        }
    }
}

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, AITag>,        
    );

    fn run(&mut self, (mut velocities, transforms, tags): Self::SystemData) {
        let summation = (&velocities, &transforms, &tags).join() 
        .map(|(_, transform, _)| {
            let result = (&velocities, &transforms).join()
            .filter(move |(_, o_transform)| {
                let dx = o_transform.translation().x -  transform.translation().x;
                let dy = o_transform.translation().y -  transform.translation().y;

                dx * dx + dy * dy <= RANGE
            }).map(|(o_vel, _)| {
                (o_vel.rotation, o_vel.speed)
            }).fold((0.0, 0.0, 0), |(tr, ts, counter), (r, s)| {
                (tr + r, ts + s, counter + 1)
            });

            (result.0 / result.2 as f32, result.1 / result.2 as f32)
        }).collect::<Vec<_>>();

        for ((velocity, _, _), (average_rot, average_spd)) in (&mut velocities, &transforms, &tags).join().zip(summation) {
            velocity.speed = velocity.speed * 0.7 + average_spd * 0.3;
            velocity.rotation = velocity.rotation * 0.7 + average_rot * 0.3;

            limit_velocity(velocity);
        }
    }
}

fn limit_velocity(velocity: &mut Velocity) {
    if velocity.speed >= 3.0 {
        velocity.speed = 3.0;
    }
    if velocity.speed < 0.0 {
        velocity.speed = 0.0;
    }
    
    if velocity.rotation < 0.0 {
        velocity.rotation += TWO_PI;
    }

    if velocity.rotation >= TWO_PI {
        velocity.rotation -= TWO_PI;
    }
}