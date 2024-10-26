mod demos;

// use demos::random_walk::RandomWalkPlugin;
// use demos::random_walk_bounded::BoundedRandomWalkPlugin;
// use demos::random_walk::RandomWalkPlugin;
// use demos::levy_flight::LevyFlightPlugin;
use demos::perlin_walker::PerlinWalkerPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(LevyFlightPlugin)
        .add_plugins(PerlinWalkerPlugin)
        .run();
}
