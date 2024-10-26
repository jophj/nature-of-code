use bevy::{color::palettes::css::WHITE, prelude::*};
use noise::{NoiseFn, Perlin};

fn draw_walk(mut windows: Query<&mut Window>, time: Res<Time>, mut gizmos: Gizmos) {
    let window = windows.iter_mut().next().unwrap();
    let offset: f64 = (time.elapsed_seconds() / 1.0) as f64;
    let v = Perlin::new(0);
    let mut input = 0_f64 + offset;
    let signal_0 = v.get([input, 0.0]) * 120_f64;
    let signal_1 = v.get([input * 8.0 + 69420.0, 0.0]) * 12_f64;
    let signal_2 = v.get([input / 20.0 + 694200.0, 0.0]) * 120_f64 as f64;
    let value = signal_0 + signal_1 + signal_2;
    let mut start = Vec2::new(-window.width() / 2.0, value as f32);
    for i in 1..(window.width() as i32) {
        input = input + 0.007;
        let signal_0 = v.get([input, 0.0]) * 120_f64;
        let signal_1 = v.get([input * 8.0 + 69420.0, 0.0]) * 12_f64;
        let signal_2 = v.get([input / 20.0 + 694200.0, 0.0]) * 120_f64 as f64;
        let value = signal_0 + signal_1 + signal_2;
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
