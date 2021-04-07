use crate::geom::{Vector3, Triangle};

fn wrap(n: f64, max: usize) -> f64 {
    if n < 0.0 {
        n + max as f64
    } else if n >= max as f64 {
        n - max as f64
    } else {
        n
    }
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

    pub fn draw(&mut self, mut x: f64, mut y: f64, color: u32) {
        if self.wrap_pixels {
            x = wrap(x, self.width);
            y = wrap(y, self.height);
        }
        let idx = y.round() as usize * self.width + x.round() as usize;
        if idx < self.width * self.height {
            self.buffer[idx] = color;
        }
    }

    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: u32) {
        let mut x: f64;
        let mut y: f64;
        let xe: f64;
        let ye: f64;
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2.0 * dy1 - dx1;
        let mut py = 2.0 * dx1 - dy1;

        if dy1 <= dx1 {
            if dx >= 0.0 {
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
                x += 1.0;
                if px < 0.0 {
                    px += 2.0 * dy1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        y += 1.0;
                    } else {
                        y -= 1.0;
                    }
                    px += 2.0 * (dy1 - dx1);
                }
                self.draw(x, y, color);
            }
        } else {
            if dy >= 0.0 {
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
                y += 1.0;
                if py <= 0.0 {
                    py += 2.0 * dx1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        x += 1.0;
                    } else {
                        x -= 1.0;
                    }
                    py += 2.0 * (dx1 - dy1);
                }
                self.draw(x, y, color);
            }
        }
    }
    
    pub fn draw_triangle(&mut self, tri: Triangle, color: u32) {
        self.draw_line(tri.0.x, tri.0.y, tri.1.x, tri.1.y, color);
        self.draw_line(tri.1.x, tri.1.y, tri.2.x, tri.2.y, color);
        self.draw_line(tri.2.x, tri.2.y, tri.0.x, tri.0.y, color);
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
                new_points[i % n_points].0,
                new_points[i % n_points].1,
                new_points[j % n_points].0,
                new_points[j % n_points].1,
                color,
            );
        }
    }

    pub fn draw_text(&mut self, text: &str, x: f64, y: f64, color: u32) {
        let bitmap = bitfont::bitmap_bool(text).unwrap();
        for (cy, row) in bitmap.iter().enumerate() {
            for (cx, chr) in row.iter().enumerate() {
                if *chr {
                    self.draw(x + cx as f64, y + cy as f64, color);
                }
            }
        }
    }
}
