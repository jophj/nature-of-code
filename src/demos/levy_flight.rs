use bevy::{
    color::palettes::css::WHITE, prelude::*
};
use rand::{thread_rng, prelude::*};

const SCALE: f32 = 1.0;
const BOUNDARIES: Vec2 = Vec2::new(256., 128.);
const MAX_STEP_SIZE: f32 = 20.0;

#[derive(Component)]
struct Walk {
    path: Vec<Vec3>,
}
impl Walk {
    fn new() -> Self {
        Walk { path: Vec::new() }
    }

    fn append(&mut self, step: Vec3) {
        self.path.push(step);
    }
}

trait Walker {
    fn walk(path: &Vec<Vec3>, boundaries: &Vec2) -> Vec3;
}

struct LevyFlightWalker;
impl Walker for LevyFlightWalker {
    fn walk(path: &Vec<Vec3>, boundaries: &Vec2) -> Vec3 {
        let mut rng = thread_rng();

        // step direction
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);

        // step size
        let mut r1 = rng.gen_range(0_f32..=1.0);
        let mut r2 = rng.gen_range(0_f32..=1.0);
        while r2.powi(2) > r1 {
            r1 = rng.gen_range(0_f32..=1.0);
            r2 = rng.gen_range(0_f32..=1.0);
        }
        let s = r1 * MAX_STEP_SIZE;

        // TODO restore this computation
        let resultant = path.iter().fold(Vec2::new(0., 0.), |acc, current| Vec2::new(acc.x + current.x * s, acc.y + current.y * s));

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

        Vec3::new(x, y, s)
    }
}


// draw lines based on the RandomWalk component
fn walk(mut gizmos: Gizmos, mut query: Query<&mut Walk>) {
    // log the window size
    // info!("Window size: {:?}", window);
    for mut walker in query.iter_mut() {
        let step = LevyFlightWalker::walk(&walker.path, &BOUNDARIES);
        walker.append(step);
        let path = &walker.path;
        let mut start = Vec2::new(0., 0.);
        for i in path.iter() {
            let end = Vec2::new(start.x + i.x * SCALE * i.z, start.y + i.y * SCALE * i.z);
            gizmos.line_2d(start, end,  WHITE);
            start = end;
            // info!("Start: {:?}, End: {:?}", start, end);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Walk::new());
    commands.spawn(
        TextBundle::from_section("Lévy flight", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

pub struct LevyFlightPlugin;
impl Plugin for LevyFlightPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.01))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, walk);
    }
}
