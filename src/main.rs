mod demos;

use demos::random_walk::RandomWalkPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(RandomWalkPlugin)
        .run();
}
