extern crate amethyst;

use amethyst::input::{is_close_requested, is_key_down, InputBundle};
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Event, Pipeline, PosTex, RenderBundle, Stage, VirtualKeyCode,
};
use amethyst::core::transform::TransformBundle;

mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let stage = Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat::<PosTex>::new());
    let pipe = Pipeline::build().with_stage(stage);

    let display_config_path = "./resources/display_config.ron";
    let display_config = DisplayConfig::load(&display_config_path);
    let render_bundle = RenderBundle::new(pipe, Some(display_config));
    let transform_bundle = TransformBundle::new();

    let bindings_path = format!("{}/resources/bindings_config.ron", env!("CARGO_MANIFEST_DIR"));
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(transform_bundle)?
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);
    let mut game = Application::new("./", pong::Pong, game_data)?;
    game.run();

    Ok(())
}
