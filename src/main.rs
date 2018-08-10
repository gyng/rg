extern crate amethyst;

use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex, RenderBundle, Stage, VirtualKeyCode,
};

use amethyst::ecs::Resources;
use amethyst::ecs::World;

// struct Example;

// impl<'a, 'b> State<GameData<'a, 'b>> for Example {
//     fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
//         if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
//             Trans::Quit
//         } else {
//             Trans::None
//         }
//     }

//     fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
//         data.data.update(&data.world);
//         Trans::None
//     }
// }

mod states;
use states::GameplayState;

mod resources;
use resources::MyResource;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let mut resources = Resources::new();
    let my = MyResource { game_score: 0 };
    resources.insert(my);
    let fetched = resources.try_fetch::<MyResource>().expect("no MyResource present in Resource");
    println!("Resource {:?}", &fetched.game_score);

    let mut world = World::new();
    world.add_resource(my);

    // Panics
    // let readback = world.read_resource::<MyResource>();
    // // let my = world.res.entry::<MyResource>().or_insert_with(|| MyResource);
    // println!("Read back resource {:?}", &readback.game_score);
    // let mut mutable_resource = world.write_resource::<MyResource>();
    // mutable_resource.game_score = 1;

    // let readback = world.read_resource::<MyResource>();
    // // let my = world.res.entry::<MyResource>().or_insert_with(|| MyResource);
    // println!("Read back resource {:?}", &readback.game_score);

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::build("./", GameplayState)?.build(game_data)?;
    game.run();
    Ok(())
}
