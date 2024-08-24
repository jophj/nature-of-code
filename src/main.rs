//! This example demonstrates bounding volume intersections.

use std::path;

use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::{thread_rng, prelude::*};

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(0.01))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (draw_lines))
        .run();
}

#[derive(Component)]
struct RandomWalk {
    path: Vec<Vec2>,
}

trait Walker {
    fn walk(&mut self);
}

const SCALE: f32 = 10.0;

impl Walker for RandomWalk {
    fn walk(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        self.path.push(Vec2::new(x, y));

    }
}

// draw lines based on the RandomWalk component
fn draw_lines(mut gizmos: Gizmos, mut query: Query<&mut RandomWalk>) {
    let mut walker  = query.iter_mut().next().unwrap();
    walker.walk();
    let path = &walker.path;
    let mut start = Vec2::new(0., 0.);
    for i in path.iter() {
        let end = Vec2::new(start.x + i.x * SCALE, start.y + i.y * SCALE);
        gizmos.line_2d(start, end,  WHITE);
        start = end;
    }
}

#[derive(Component)]
enum DesiredVolume {
    Aabb,
}

#[derive(Component, Deref, DerefMut, Default)]
struct Intersects(bool);

const OFFSET_X: f32 = 125.;
const OFFSET_Y: f32 = 75.;

fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
            ..default()
        },
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    let mut random_walk = RandomWalk {
        path: Vec::new()
    };

    let mut rng = thread_rng();

    for _ in 0..2 {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        random_walk.path.push(Vec2::new(x, y));
    }
    commands.spawn(random_walk);
    commands.spawn(
        TextBundle::from_section("Random walk", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}
