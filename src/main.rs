mod demos;

// use demos::random_walk::RandomWalkPlugin;
use demos::random_walk_centered::RandomWalkCenteredPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(RandomWalkCenteredPlugin)
        .run();
}
