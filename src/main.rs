mod geom;
mod canvas;

extern crate minifb;

use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};

use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use crate::geom::{Triangle, Matrix4, Mesh};
use crate::canvas::Canvas;


fn main() {
    let cube_mesh = Mesh {
        tris: vec![
            // SOUTH
            Triangle::from((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)),
            Triangle::from((0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 0.0, 0.0)),
            // EAST
            Triangle::from((1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 1.0)),
            Triangle::from((1.0, 0.0, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0, 1.0)),
            // NORTH
            Triangle::from((1.0, 0.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)),
            Triangle::from((1.0, 0.0, 1.0), (0.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
            // WEST
            Triangle::from((0.0, 0.0, 1.0), (0.0, 1.0, 1.0), (0.0, 1.0, 0.0)),
            Triangle::from((0.0, 0.0, 1.0), (0.0, 1.0, 0.0), (0.0, 0.0, 0.0)),
            // TOP
            Triangle::from((0.0, 1.0, 0.0), (0.0, 1.0, 1.0), (1.0, 1.0, 1.0)),
            Triangle::from((0.0, 1.0, 0.0), (1.0, 1.0, 1.0), (1.0, 1.0, 0.0)),
            // BOTTOM
            Triangle::from((1.0, 0.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 0.0)),
            Triangle::from((1.0, 0.0, 1.0), (0.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
        ],
    };

    // window
    let screen_width = 320;
    let screen_height = 180;

    let mut window = Window::new(
        "Pog3D",
        screen_width,
        screen_height,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut canvas = Canvas::new(screen_width, screen_height, false);

    // Projection Matrix
    let near = 0.1;
    let far = 1000.0;
    let fov = 90.0;
    let aspect_ratio = screen_height as f64 / screen_width as f64;
    let fov_rad = 1.0 / (fov * 0.5 / 180.0 * PI).tan();

    #[rustfmt::skip]
    let mat_proj: Matrix4 = [
        [ aspect_ratio * fov_rad, 0.0,     0.0,                          0.0 ],
        [ 0.0,                    fov_rad, 0.0,                          0.0 ],
        [ 0.0,                    0.0,     far / (far - near),           1.0 ],
        [ 0.0,                    0.0,     (-far * near) / (far - near), 0.0 ],
    ];
        
    let mut theta = 0.0;
        
    let mut last_update = Instant::now();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let delta = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        canvas.clear(0x000000);
        
        // Rotation matrices
        theta += delta as f64;
        let h_theta = theta * 0.5;
        #[rustfmt::skip] 
        let mat_rot_z: Matrix4 = [
            [ theta.cos(),  theta.sin(), 0.0, 0.0 ],
            [ -theta.sin(), theta.cos(), 0.0, 0.0 ],
            [ 0.0,          0.0,         1.0, 0.0 ],
            [ 0.0,          0.0,         0.0, 1.0 ],
        ];
        let mat_rot_x: Matrix4 = [
            [ 1.0, 0.0,            0.0,           0.0 ],
            [ 0.0, h_theta.cos(),  h_theta.sin(), 0.0 ],
            [ 0.0, -h_theta.sin(), h_theta.cos(), 0.0 ],
            [ 0.0, 0.0,            0.0,           1.0 ],
        ];
        
        // project triangles
        for tri in cube_mesh.tris.iter() {
            let rot_tri_z = Triangle (
                tri.0.multiply_matrix(&mat_rot_z),
                tri.1.multiply_matrix(&mat_rot_z),
                tri.2.multiply_matrix(&mat_rot_z),
            );
            
            let rot_tri_x = Triangle (
                rot_tri_z.0.multiply_matrix(&mat_rot_x),
                rot_tri_z.1.multiply_matrix(&mat_rot_x),
                rot_tri_z.2.multiply_matrix(&mat_rot_x),
            );

            let mut tran_tri = Triangle (rot_tri_x.0, rot_tri_x.1, rot_tri_x.2);
            tran_tri.0.z += 3.0;
            tran_tri.1.z += 3.0;
            tran_tri.2.z += 3.0;

            let mut proj_tri = Triangle (
                tran_tri.0.multiply_matrix(&mat_proj),
                tran_tri.1.multiply_matrix(&mat_proj),
                tran_tri.2.multiply_matrix(&mat_proj),
            );

            proj_tri.0.x += 1.0; proj_tri.0.y += 1.0;
            proj_tri.1.x += 1.0; proj_tri.1.y += 1.0;
            proj_tri.2.x += 1.0; proj_tri.2.y += 1.0;
            
            proj_tri.0.x *= 0.5 * screen_width as f64;
            proj_tri.0.y *= 0.5 * screen_height as f64;
            proj_tri.1.x *= 0.5 * screen_width as f64;
            proj_tri.1.y *= 0.5 * screen_height as f64;
            proj_tri.2.x *= 0.5 * screen_width as f64;
            proj_tri.2.y *= 0.5 * screen_height as f64;
            
            canvas.draw_triangle(proj_tri, 0xffffff);
        }
        
        /*
        canvas.draw_triangle(
            Triangle::from((10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (0.0, 10.0, 0.0)),
            0x448aff,
        );
        */

        window
            .update_with_buffer(&canvas.buffer, screen_width, screen_height)
            .unwrap();
    }
}
