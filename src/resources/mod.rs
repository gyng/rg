// A Resource is any type that stores data that you might need for your game AND that is not specific to an entity.
// For example, the score of a pong game is global to the whole game and isn't owned by any of the entities
// (paddle, ball and even the ui score text).

use amethyst::ecs::Resources;

#[derive(Copy, Clone, Debug)]
pub struct MyResource {
    pub game_score: i32,
}
