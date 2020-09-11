/*
    this is a game developed and created by ezra goldner
*/

mod cube_jump;
mod cube;
mod size;
mod systems;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
    core::TransformBundle,
    input::{InputBundle, StringBindings}
};

use crate::cube_jump::CubeJump;

const DISPLAY_CONFIG: &str = "display.ron";
const BINDING_CONFIG: &str = "binding.ron";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let config_path = root.join("config");
    let display_config_path = config_path.join(DISPLAY_CONFIG);
    let binding_config_path = config_path.join(BINDING_CONFIG);
    let assets_path = root.join("assets");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::CubeSystem {
            jumping: false,
            jump_factor: 1.0
        }, "cube_system", &["input_system"]);

    let mut game = Application::new(assets_path, CubeJump, game_data)?;

    game.run();
    Ok(())
}
