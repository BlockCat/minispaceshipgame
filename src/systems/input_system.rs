use amethyst::ecs::System;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{ WriteStorage, Read, ReadStorage, Join};
use amethyst::core::transform::{ Transform };

use crate::components::{ PlayerTag, AITag, Ship};
use crate::components::Velocity;

pub struct InputSystem;
pub struct AISystem;

const TWO_PI: f32 = std::f32::consts::PI * 2.0;
const ALIGNMENT_RANGE: f32 = 15.0;
const COHESION_RANGE: f32 = 15.0;
const SEPARATION_RANGE: f32 = 14.0;
const MAX_SPEED: f32 = 1.0;


impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Ship>,
        ReadStorage<'a, PlayerTag>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut ships, tags, input): Self::SystemData) {
        let movement = input.axis_value("speed");
        let leftright = input.axis_value("leftright");
        for (velocity, ship, _) in (&mut transforms, &mut ships, &tags).join() {

            if let Some(lr) = leftright {
                ship.rotation += lr * 0.1;
            }

            if let Some(mv) = movement {                
                let dx = ship.rotation.cos();
                let dy = ship.rotation.sin();

                if mv > 0.0 {
                    velocity.dx += dx * 0.1;
                    velocity.dy += dy * 0.1;
                }
                if mv < 0.0 {
                    velocity.dx -= dx * 0.2;
                    velocity.dy -= dy * 0.2;
                }                
            }           

            limit_velocity(velocity);
            limit_rotation(ship);
            
        }
    }
}

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Ship>,
        ReadStorage<'a, AITag>,        
    );

    fn run(&mut self, (mut velocities, mut ships, tags): Self::SystemData) {
        let alignment = (&velocities, &ships, &tags).join() 
        .map(|(_, ship, _)| {
            let result = (&velocities, &ships).join()
            .filter(move |(_, o_ship)| {
                o_ship.id != ship.id && distance_squared(o_ship, ship) <= ALIGNMENT_RANGE * ALIGNMENT_RANGE
            }).map(|(o_vel, o_ship)| {
                let multiplier = if o_ship.id == 0 { 10.0} else { 1.0 };
                (o_vel.dx * multiplier, o_vel.dy * multiplier)
            }).fold((0.0, 0.0, 0), |(total_dx, total_dy, counter), (dx, dy)| {
                (total_dx + dx, total_dy + dy, counter + 1)
            });
            if result.2 == 0 { 
                (0.0, 0.0)
            } else { 
                (result.0 / result.2 as f32, result.1 / result.2 as f32).normalize().unwrap_or((0.0, 0.0))
            }
        }).collect::<Vec<(f32, f32)>>();

        let cohesion = (&velocities, &ships, &tags).join() 
        .map(|(_, ship, _)| {
            let result = (&velocities, &ships).join()
            .filter(move |(_, o_ship)| {
                o_ship.id != ship.id && distance_squared(o_ship, ship) <= COHESION_RANGE * COHESION_RANGE
            }).map(|(_, o_ship)| {
                (o_ship.x, o_ship.y)
            }).fold((0.0, 0.0, 0), |(total_x, total_y, counter), (x, y)| {
                (total_x + x, total_y + y, counter + 1)
            });
            if result.2 == 0 { 
                (0.0, 0.0)
            } else { 
                (result.0 / result.2 as f32 - ship.x, result.1 / result.2 as f32 - ship.y).normalize().unwrap_or((0.0, 0.0))
            }
            
        }).collect::<Vec<(f32, f32)>>();

        let separation = (&velocities, &ships, &tags).join() 
        .map(|(_, ship, _)| {
            let my_x = ship.x;
            let my_y = ship.y;
            let result = (&velocities, &ships).join()
            .filter(move |(_, o_ship)| {
                ship.id != o_ship.id && distance_squared(o_ship, ship) <= SEPARATION_RANGE * SEPARATION_RANGE
            }).map(|(_, o_ship)| {
                (o_ship.x - my_x, o_ship.y - my_y)
            }).fold((0.0, 0.0), |(total_x, total_y), (x, y)| {
                (total_x + x, total_y + y)
            });
            (-result.0, -result.1).normalize().unwrap_or((0.0, 0.0))
        }).collect::<Vec<(f32, f32)>>();

        for ((((velocity, ship, _), (adx, ady)), (cx, cy)), (sdx, sdy)) in (&mut velocities, &mut ships, &tags).join().zip(alignment).zip(cohesion).zip(separation) {
            let (cohesion_dx, cohesion_dy) = (cx - ship.x, cy - ship.y).normalize().unwrap_or((0.0, 0.0)); 
            
            let cohesion_weight = 1.0;
            let alignment_weight = 1.0;
            let separation_weight = 1.0;

            let dx = adx * alignment_weight + cohesion_dx * cohesion_weight + sdx * separation_weight;
            let dy = ady * alignment_weight + cohesion_dy * cohesion_weight + sdy * separation_weight;

            velocity.dx += dx;
            velocity.dy += dy;
            velocity.normalize();
            ship.rotation = velocity.dy.atan2(velocity.dx);

            limit_velocity(velocity);
            limit_rotation(ship);
        }
    }
}

fn limit_velocity(velocity: &mut Velocity) {
    let speed_square = velocity.dx.powi(2) + velocity.dy.powi(2);
    
    if speed_square > MAX_SPEED * MAX_SPEED {
        let speed = MAX_SPEED / speed_square.sqrt();
        velocity.dx *= speed;
        velocity.dy *= speed;
    }
    
   
}

fn limit_rotation(ship: &mut Ship) {
     if ship.rotation < 0.0 {
        ship.rotation += TWO_PI;
    }

    if ship.rotation >= TWO_PI {
        ship.rotation -= TWO_PI;
    }
}

fn distance_squared(a: &Ship, b: &Ship) -> f32 {
    let (width, height) = (crate::ARENA_WIDTH, crate::ARENA_HEIGHT);
    let (ax, ay): (f32, f32) = (a.x, a.y);
    let (bx, by): (f32, f32) = (b.x, b.y);

    let min_x = if ax < bx { ax } else { bx };
    let max_x = if ax > bx { ax } else { bx };

    let min_y = if ay < by { ay } else { by };
    let max_y = if ay > by { ay } else { by };
    
    let dx1 = max_x - min_x;
    let dx2 = min_x + (width - max_x);

    let dy1 = max_y - min_y;
    let dy2 = min_y + (height - max_y);

    let min_dx = if dx1 < dx2 { dx1 } else { dx2 };
    let min_dy = if dy1 < dy2 { dy1 } else { dy2 };

    min_dx.powi(2) + min_dy.powi(2)
}

trait Normalize {
    fn length(&self) -> f32;
    fn normalize(&self) -> Result<Self, ()> where Self: Sized;
}

impl Normalize for (f32, f32) {
    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn normalize(&self) -> Result<Self, ()> {
        let distance = self.length();

        if distance > 0.0 {
            Ok((self.0 / distance, self.1 / distance))
        } else { 
            Err(())
        }
    }
}