use crate::size::Size;
use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Cube {
    pub size: Size
}

impl Cube {
    pub(crate) fn new(width: u8, height: u8) -> Cube {
        Cube {
            size: Size::new(width as usize, height as usize)
        }
    }
}

impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}