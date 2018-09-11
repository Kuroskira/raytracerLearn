use vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub a: Vector3,
    pub b: Vector3,
}

impl Ray {
    pub fn origin(&self) -> Vector3 {
        self.a
    }

    pub fn direction(&self) -> Vector3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3 {
        self.origin() + t * self.direction()
    }
}

