use bevy::{color::palettes::css::WHITE, prelude::*, render::view::window};
use noise::{NoiseFn, Perlin};

fn draw_walk(mut windows: Query<&mut Window>, time: Res<Time>, mut gizmos: Gizmos) {
    // log the window size
    // info!("Window size: {:?}", window);
    // println!("Time: {:?} {}", time.delta_seconds(), time.elapsed_seconds());
    // for from 0 to boundaries width
    let window = windows.iter_mut().next().unwrap();
    let t: f64 = (time.elapsed_seconds() / 1.0) as f64;
    let v = Perlin::new(0);
    let mut input = 0_f64 + t;
    let start_value = v.get([input]) * 120_f64;
    let mut start = Vec2::new(-window.width() / 2.0, start_value as f32);
    println!("Start: {:?} {}", input, start_value);
    for i in 1..(window.width() as i32) {
        input = input + 0.007;
        let value = v.get([input]) * 120_f64;
        // println!("Value: {} {}", input, value);
        let end = Vec2::new((i as f32) - window.width() / 2.0, value as f32);
        gizmos.line_2d(start, end, WHITE);
        start = end;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section("Perlin Walker", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

pub struct PerlinWalkerPlugin;
impl Plugin for PerlinWalkerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(0.016))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, draw_walk);
    }
}
