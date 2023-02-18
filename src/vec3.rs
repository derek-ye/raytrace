pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.lenth_squared().sqrt()
    }
    fn lenth_squared(&self) -> f32{
        self.x * self.x 
        + self.y * self.y
        + self.z * self.z
    }

}