use bevy::{
    color::palettes::css::WHITE, prelude::*
};
use rand::{thread_rng, prelude::*};

const SCALE: f32 = 10.0;

#[derive(Component)]
struct RandomWalk {
    path: Vec<Vec2>,
}

impl RandomWalk {
    fn new() -> Self {
        RandomWalk { path: Vec::new() }
    }

    fn walk(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        self.path.push(Vec2::new(x, y));

    }
}

// draw lines based on the RandomWalk component
fn walk(mut gizmos: Gizmos, mut query: Query<&mut RandomWalk>, windows: Query<&Window>) {
    // log the window size
    let window = windows.iter().next().unwrap();
    // info!("Window size: {:?}", window);
    for mut walker in query.iter_mut() {
        walker.walk();
        let path = &walker.path;
        let mut start = Vec2::new(0., 0.);
        for i in path.iter() {
            let mut end = Vec2::new(start.x + 1. * SCALE, start.y + 1. * SCALE);
            match end {
                Vec2 { x, y } if x < -(window.physical_width() as f32 ) / 2. - 100. => end = Vec2::new(start.x - i.x * SCALE, end.y),
                Vec2 { x, y } if x > window.physical_width() as f32 / 2. - 100. => end = Vec2::new(start.x - i.x * SCALE, end.y),
                Vec2 { x, y } if y < -(window.physical_height() as f32 ) / 2. - 100. => end = Vec2::new(end.x, start.y - i.y * SCALE),
                Vec2 { x, y } if y > window.physical_height() as f32  / 2. - 100. => end = Vec2::new(end.x, start.y - i.y * SCALE),
                _ => {}
            }
            gizmos.line_2d(start, end,  WHITE);
            start = end;
            info!("Start: {:?}, End: {:?}", start, end);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(RandomWalk::new());
    commands.spawn(RandomWalk::new());
    commands.spawn(
        TextBundle::from_section("Random walk", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

pub struct RandomWalkCenteredPlugin;
impl Plugin for RandomWalkCenteredPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.01))
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, walk);
    }
}