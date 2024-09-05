use bevy::{
    color::palettes::css::WHITE, prelude::*
};
use rand::{thread_rng, prelude::*};

const SCALE: f32 = 10.0;
const BOUNDARIES: Vec2 = Vec2::new(22., 22.);

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
    fn walk(path: &Vec<Vec2>, boundaries: &Vec2) -> Vec2;
}

struct BoundedRandomWalker;
impl Walker for BoundedRandomWalker {
    fn walk(path: &Vec<Vec2>, boundaries: &Vec2) -> Vec2 {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);

        let resultant = path.iter().fold(Vec2::new(0., 0.), |acc, x| Vec2::new(acc.x + x.x, acc.y + x.y));

        let x: f32  = if resultant.x < -boundaries.x || resultant.x > boundaries.x {
            // move towards the center
            if resultant.x > 0. {
                -1.0
            } else {
                1.0
            }
        } else {
            // move in the same direction
            x
        };

        let y = if resultant.y < -boundaries.y || resultant.y > boundaries.y {
            // move towards the center
            if resultant.y > 0. {
                -1.0
            } else {
                1.0
            }
        } else {
            // move in the same direction
            y
        };

        Vec2::new(x as f32, y as f32)
    }
}


// draw lines based on the RandomWalk component
fn walk(mut gizmos: Gizmos, mut query: Query<&mut Walk>) {
    // log the window size
    // info!("Window size: {:?}", window);
    for mut walker in query.iter_mut() {
        let step = BoundedRandomWalker::walk(&walker.path, &BOUNDARIES);
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
        TextBundle::from_section("Bounded random walker", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

pub struct BoundedRandomWalkPlugin;
impl Plugin for BoundedRandomWalkPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.01))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, walk);
    }
}
