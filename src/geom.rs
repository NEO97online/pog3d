pub type Matrix4 = [[f64; 4]; 4];

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from(xyz: (f64, f64, f64)) -> Vector3 {
        Vector3 {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
        }
    }
    
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn multiply_matrix(&self, mat: &Matrix4) -> Vector3 {
        let mut x = self.x * mat[0][0] + self.y * mat[1][0] + self.z * mat[2][0] + mat[3][0];
        let mut y = self.x * mat[0][1] + self.y * mat[1][1] + self.z * mat[2][1] + mat[3][1];
        let mut z = self.x * mat[0][2] + self.y * mat[1][2] + self.z * mat[2][2] + mat[3][2];
        let divis = self.x * mat[0][3] + self.y * mat[1][3] + self.z * mat[2][3] + mat[3][3];

        if divis != 0.0 {
            x /= divis;
            y /= divis;
            z /= divis;
        }

        Vector3 { x, y, z }
    }
}

pub struct Triangle (pub Vector3, pub Vector3, pub Vector3);

impl Triangle {
    pub fn from(p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> Triangle {
        Triangle(
            Vector3::from(p1),
            Vector3::from(p2),
            Vector3::from(p3)
        )
    }
    
    pub fn zero() -> Triangle {
        Triangle(Vector3::zero(), Vector3::zero(), Vector3::zero())
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}