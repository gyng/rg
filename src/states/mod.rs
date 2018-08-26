// use amethyst::input::is_key_down;
// use amethyst::prelude::*;
// use amethyst::renderer::{Event, VirtualKeyCode};
// use amethyst::{State, StateData};

// // struct GameplayState {
// //     player_count: u8,
// // }

// // impl<'a, 'b> State<GameData<'a, 'b>> for GameplayState {
// //     fn on_start(&mut self, _data: StateData<'a, 'b>>) {
// //         println!("Number of players: {}", self.player_count);
// //     }
// // }

// pub struct GameplayState;
// pub struct PausedState;

// impl<'a, 'b> State<GameData<'a, 'b>> for GameplayState {
//     fn handle_event(
//         &mut self,
//         _data: StateData<GameData>,
//         event: Event,
//     ) -> Trans<GameData<'a, 'b>> {
//         if is_key_down(&event, VirtualKeyCode::Escape) {
//             println!("pausing!");
//             return Trans::Push(Box::new(PausedState));
//         }

//         Trans::None
//     }

//     fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
//         data.data.update(&data.world);
//         // data
//         Trans::None
//     }
// }

// impl<'a, 'b> State<GameData<'a, 'b>> for PausedState {
//     fn handle_event(
//         &mut self,
//         _data: StateData<GameData>,
//         event: Event,
//     ) -> Trans<GameData<'a, 'b>> {
//         if is_key_down(&event, VirtualKeyCode::Escape) {
//             println!("unpausing!");
//             return Trans::Pop;
//         }

//         Trans::None
//     }

//     fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
//         data.data.update(&data.world);
//         Trans::None
//     }
// }
