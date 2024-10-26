use std::f32::consts::PI;

/* Function to linearly interpolate between a0 and a1
 * Weight w should be in the range [0.0, 1.0]
 */
fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    // Uncomment for clamping:
    // if w < 0.0 { return a0; }
    // if w > 1.0 { return a1; }

    (a1 - a0) * w + a0
    // Uncomment for cubic interpolation (Smoothstep)
    // (a1 - a0) * (3.0 - 2.0 * w) * w * w + a0
    // Uncomment for Smootherstep
    // (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

#[derive(Debug, Copy, Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

/* Create pseudorandom direction vector */
fn random_gradient(ix: i32, iy: i32) -> Vector2 {
    let w = 8 * std::mem::size_of::<u32>() as u32;
    let s = w / 2;
    let mut a = ix as u32;
    let mut b = iy as u32;
    a = a.wrapping_mul(3284157443);
    b ^= a << s | a >> (w - s);
    b = b.wrapping_mul(1911520717);
    a ^= b << s | b >> (w - s);
    a = a.wrapping_mul(2048419325);

    let random_angle = a as f32 * (PI / (u32::MAX as f32));
    Vector2 {
        x: random_angle.cos(),
        y: random_angle.sin(),
    }
}

// Computes the dot product of the distance and gradient vectors.
fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    // Get gradient from integer coordinates
    let gradient = random_gradient(ix, iy);

    // Compute the distance vector
    let dx = x - ix as f32;
    let dy = y - iy as f32;

    // Compute the dot-product
    dx * gradient.x + dy * gradient.y
}

// Compute Perlin noise at coordinates x, y
fn perlin(x: f32, y: f32) -> f32 {
    // Determine grid cell coordinates
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    // Determine interpolation weights
    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    // Interpolate between grid point gradients
    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);

    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);

    interpolate(ix0, ix1, sy)
}

fn main() {
    let x = 1.3;
    let y = 4.7;
    println!("Perlin noise value at ({}, {}): {}", x, y, perlin(x, y));
}
