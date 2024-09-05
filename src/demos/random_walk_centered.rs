use std::path;

use bevy::{
    color::palettes::css::WHITE, prelude::*
};
use rand::{thread_rng, prelude::*};

const SCALE: f32 = 10.0;

#[derive(Component)]
struct Walk {
    path: Vec<Vec2>,
}
impl Walk {
    fn new() -> Self {
        Walk { path: Vec::new() }
    }

    fn append(&mut self, step: Vec2) {
        self.path.push(step);
    }
}

trait Walker {
    fn walk(path: &Vec<Vec2>, window: &Window) -> Vec2;
}

struct RandomWalker;
impl Walker for RandomWalker {
    fn walk(_path: &Vec<Vec2>, _window: &Window) -> Vec2 {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);

        Vec2::new(x as f32, y as f32)
    }
}


// draw lines based on the RandomWalk component
fn walk(mut gizmos: Gizmos, mut query: Query<&mut Walk>, windows: Query<&Window>) {
    // log the window size
    let window = windows.iter().next().unwrap();
    // info!("Window size: {:?}", window);
    for mut walker in query.iter_mut() {
        let step = RandomWalker::walk(&walker.path, window);
        walker.append(step);
        let path = &walker.path;
        let mut start = Vec2::new(0., 0.);
        for i in path.iter() {
            let end = Vec2::new(start.x + i.x * SCALE, start.y + i.y * SCALE);
            // match end {
            //     Vec2 { x, y } if x < -(window.physical_width() as f32 ) / 2. - 100. => end = Vec2::new(start.x - i.x * SCALE, end.y),
            //     Vec2 { x, y } if x > window.physical_width() as f32 / 2. - 100. => end = Vec2::new(start.x - i.x * SCALE, end.y),
            //     Vec2 { x, y } if y < -(window.physical_height() as f32 ) / 2. - 100. => end = Vec2::new(end.x, start.y - i.y * SCALE),
            //     Vec2 { x, y } if y > window.physical_height() as f32  / 2. - 100. => end = Vec2::new(end.x, start.y - i.y * SCALE),
            //     _ => {}
            // }
            gizmos.line_2d(start, end,  WHITE);
            start = end;
            // info!("Start: {:?}, End: {:?}", start, end);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Walk::new());
    commands.spawn(Walk::new());
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
