use crate::geom::{Vector3, Triangle};

fn wrap(n: i64, max: usize) -> i64 {
    if n < 0 {
        n + max as i64
    } else if n >= max as i64 {
        n - max as i64
    } else {
        n
    }
}

fn swap<T: Copy>(n1: &mut T, n2: &mut T) {
    let t = *n1;
    *n1 = *n2;
    *n2 = t;
}

pub struct Canvas {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub wrap_pixels: bool,
}

impl Canvas {
    pub fn new(width: usize, height: usize, wrap_pixels: bool) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height],
            wrap_pixels,
        }
    }

    pub fn clear(&mut self, color: u32) {
        for i in self.buffer.iter_mut() {
            *i = color;
        }
    }

    pub fn draw(&mut self, mut x: i64, mut y: i64, color: u32) {
        if self.wrap_pixels {
            x = wrap(x, self.width);
            y = wrap(y, self.height);
        }
        if x > 0 && y > 0 {
            let idx = y as usize * self.width + x as usize;
            if idx < self.width * self.height {
                self.buffer[idx] = color;
            }
        }
    }

    pub fn draw_line(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, color: u32) {
        let mut x: i64;
        let mut y: i64;
        let xe: i64;
        let ye: i64;
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1;
                y = y1;
                xe = x2;
            } else {
                x = x2;
                y = y2;
                xe = x1;
            }

            self.draw(x, y, color);

            while x < xe {
                x += 1;
                if px < 0 {
                    px += 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                self.draw(x, y, color);
            }
        } else {
            if dy >= 0 {
                x = x1;
                y = y1;
                ye = y2;
            } else {
                x = x2;
                y = y2;
                ye = y1;
            }

            self.draw(x, y, color);

            while y < ye {
                y += 1;
                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }
                self.draw(x, y, color);
            }
        }
    }
    
    pub fn draw_triangle(&mut self, tri: Triangle, color: u32) {
        self.draw_line(tri.0.x as i64, tri.0.y as i64, tri.1.x as i64, tri.1.y as i64, color);
        self.draw_line(tri.1.x as i64, tri.1.y as i64, tri.2.x as i64, tri.2.y as i64, color);
        self.draw_line(tri.2.x as i64, tri.2.y as i64, tri.0.x as i64, tri.0.y as i64, color);
    }
    
    // the triangle MUST be sorted first!
    fn fill_flat_bottom_triangle(&mut self, tri: Triangle, color: u32) {
        let invslope1 = (tri.1.x - tri.0.x) / (tri.1.y - tri.0.y);
        let invslope2 = (tri.2.x - tri.0.x) / (tri.2.y - tri.0.y);
        
        let mut curx1 = tri.0.x;
        let mut curx2 = tri.0.x;
        
        for scanline_y in tri.0.y as i64..tri.1.y as i64 {
            self.draw_line(curx1 as i64, scanline_y, curx2 as i64, scanline_y, color);
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }
    
    // the triangle MUST be sorted first!
    fn fill_flat_top_triangle(&mut self, tri: Triangle, color: u32) {
        println!("{:?}", tri);
        let invslope1 = (tri.2.x - tri.0.x) / (tri.2.y - tri.0.y);
        let invslope2 = (tri.2.x - tri.1.x) / (tri.2.y - tri.1.y);

        let mut curx1 = tri.2.x;
        let mut curx2 = tri.2.x;
        
        for scanline_y in (tri.0.y as i64..tri.2.y as i64).rev() {
            self.draw_line(curx1 as i64, scanline_y, curx2 as i64, scanline_y, color);
            curx1 -= invslope1;
            curx2 -= invslope2;
        }
    }
    
    pub fn fill_triangle(&mut self, mut tri: Triangle, color: u32) {
        // sort triangle vertices in ascending y
        if tri.0.y > tri.1.y {
            swap(&mut tri.0.y, &mut tri.1.y);
            swap(&mut tri.0.x, &mut tri.1.x);
        }
        if tri.0.y > tri.2.y {
            swap(&mut tri.0.y, &mut tri.2.y);
            swap(&mut tri.0.x, &mut tri.2.x);
        }
        if tri.1.y > tri.2.y {
            swap(&mut tri.1.y, &mut tri.2.y);
            swap(&mut tri.1.x, &mut tri.2.x);
        }
        
        // fill triangle
        if tri.1.y == tri.2.y {
            // flat-bottom triangle
            self.fill_flat_bottom_triangle(tri, color);
        } else if tri.0.y == tri.1.y {
            // flat-top triangle
            self.fill_flat_top_triangle(tri, color);
        } else {
            // non-flat triangle - split into two flat triangles
            let v4x = tri.0.x + ((tri.1.y - tri.0.y) / (tri.2.y - tri.0.y)) * (tri.2.x - tri.0.x);
            let v4 = Vector3::new(v4x, tri.1.y, 0.0);
            self.fill_flat_bottom_triangle(Triangle (tri.0, tri.1, v4), color);
            self.fill_flat_top_triangle(Triangle (tri.1, v4, tri.2), color);
            //self.draw_triangle(Triangle (tri.0, tri.1, v4), 0x00ff00);
            //self.draw_triangle(Triangle (tri.1, v4, tri.2), 0xff0000);
        }
    }

    pub fn draw_wireframe_model(
        &mut self,
        points: &Vec<(f64, f64)>,
        x: f64,
        y: f64,
        rot: f64,
        scale: f64,
        color: u32,
    ) {
        let mut new_points = points.clone();
        let n_points = points.len();

        // rotate
        for i in 0..n_points {
            new_points[i].0 = points[i].0 * rot.cos() - points[i].1 * rot.sin();
            new_points[i].1 = points[i].0 * rot.sin() + points[i].1 * rot.cos();
        }

        // scale
        for i in 0..n_points {
            new_points[i].0 *= scale;
            new_points[i].1 *= scale;
        }

        // translate
        for i in 0..n_points {
            new_points[i].0 += x;
            new_points[i].1 += y;
        }

        // draw
        for i in 0..(n_points + 1) {
            let j = i + 1;
            self.draw_line(
                new_points[i % n_points].0 as i64,
                new_points[i % n_points].1 as i64,
                new_points[j % n_points].0 as i64,
                new_points[j % n_points].1 as i64,
                color,
            );
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i64, y: i64, color: u32) {
        let bitmap = bitfont::bitmap_bool(text).unwrap();
        for (cy, row) in bitmap.iter().enumerate() {
            for (cx, chr) in row.iter().enumerate() {
                if *chr {
                    self.draw(x + cx as i64, y + cy as i64, color);
                }
            }
        }
    }
}
