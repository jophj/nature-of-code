//! This example demonstrates bounding volume intersections.

use bevy::{color::palettes::css::*, prelude::*};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, render_shapes))
        .add_systems(FixedUpdate, draw_lines)
        .run();
}

#[derive(Component)]
struct Spin;

fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() / 5.);
    }
}

#[derive(Component)]
enum Shape {
    Line(Segment2d),
}

#[derive(Component)]
struct RandomWalk {
    path: Vec<Dir2>,
}

#[derive(Component)]
struct Walker;

// draw lines based on the RandomWalk component
fn draw_lines(mut gizmos: Gizmos, query: Query<(&RandomWalk)>) {
    let scale = 10.;
    let meme = query.iter().next().unwrap();
    let path = &meme.path;
    let mut start = Vec2::new(0., 0.);
    for i in path.iter() {
        let end = Vec2::new(start.x + i.x * scale, start.y + i.y * scale);
        gizmos.line_2d(start, end,  WHITE);
        start = end;
    }
}

fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match shape {
            Shape::Line(l) => {
                gizmos.primitive_2d(l, translation, rotation, color);
            }
        }
    }
}

#[derive(Component)]
enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Deref, DerefMut, Default)]
struct Intersects(bool);

const OFFSET_X: f32 = 125.;
const OFFSET_Y: f32 = 75.;


fn setup(mut commands: Commands) {
    let mut rng = thread_rng();

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
            ..default()
        },
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    let mut randomWalk = RandomWalk {
        path: Vec::new()
    };
    let vs = vec![
        Dir2::from_xy(0., 1.),
        Dir2::from_xy(0., -1.),
        Dir2::from_xy(1., 0.),
        Dir2::from_xy(-1., 0.),
    ];
    for _ in 0..10 {
        let random = vs.choose(&mut rand::thread_rng()).unwrap();
        randomWalk.path.push(*random.as_ref().unwrap())
    }
    commands.spawn((
        Walker,
        randomWalk
    ));
    commands.spawn(
        TextBundle::from_section("", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}
