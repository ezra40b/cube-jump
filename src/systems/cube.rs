use amethyst::{
    core::{
        timing::Time,
        Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::cube_jump::Cube;
use amethyst::core::num::Signed;

pub const JUMP_FACTOR: f32 = 0.85;
pub const JUMP_THRESHOLD: f32 = 0.05;

#[derive(SystemDesc)]
pub struct CubeSystem {
    pub(crate) jumping: bool,
    pub(crate) jump_factor: f32
}

impl<'s> System<'s> for CubeSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cube>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, cubes, input, time): Self::SystemData) {
        for (_cube, transform) in (&cubes, &mut transforms).join() {
            if self.jumping {
                transform.prepend_translation_y(500.0 * self.jump_factor * time.delta_seconds());
                if self.jump_factor.is_positive() {
                    self.jump_factor *= JUMP_FACTOR;
                    if self.jump_factor <= JUMP_THRESHOLD {
                        self.jump_factor = -JUMP_THRESHOLD;
                    }
                }
                else {
                    if self.jump_factor <= -JUMP_FACTOR && transform.translation().y <= 0.0 {
                        self.jumping = false;
                        transform.set_translation_y(0.0);
                    }
                    self.jump_factor /= JUMP_FACTOR;
                }
            }
            else if input.action_is_down("jump").unwrap() {
                self.jumping = true;
                self.jump_factor = 1.0;
            }
        }
    }
}