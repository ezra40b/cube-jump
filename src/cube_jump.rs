use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub use crate::cube::Cube;

pub const MAP_HEIGHT: f32 = 400.0;
pub const MAP_WIDTH: f32 = 500.0;

pub const CUBE_SIZE: u8 = 20;

fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/cube_jump_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/cube_jump_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage
    )
}

fn init_entities(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);  // paddle is the first sprite in the sprite_sheet

    world.create_entity()
        .with(sprite_render)
        .with(Cube::new(CUBE_SIZE, CUBE_SIZE))
        .with(Transform::default())
        .build();
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(MAP_WIDTH, MAP_HEIGHT))
        .with(transform)
        .build();
}

pub struct CubeJump;

impl SimpleState for CubeJump {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_spritesheet(world);

        init_entities(world, sprite_sheet_handle);
        init_camera(world);
    }
}