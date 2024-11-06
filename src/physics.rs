use std::ops::{AddAssign, MulAssign, SubAssign};

pub struct Physics{

}

pub struct Kinetics {
    pos: Vector,
    vel: Vector,
    accel: Vector,
    mass: f32,
}

impl Kinetics{
    pub fn new(mass: f32) -> Self{
        Self { pos: Vector::new(), vel: Vector::new(), accel: Vector::new(), mass }
    }

    pub fn update(&mut self, applied_forces: Vec<Vector>, field: Field, dt: f32){
        self.verlet_integrate(applied_forces, field, dt);
    }

    fn verlet_integrate(&mut self, applied_forces: Vec<Vector>, field: Field, dt: f32){
        self.accel.mul(0.5 * dt);
        self.vel += self.accel;
        self.pos += self.vel;
        self.accel = self.get_accleration(applied_forces, field);
        self.accel.mul(0.5 * dt);
        self.vel += self.accel;
    }

    fn get_accleration(&mut self, applied_forces: Vec<Vector>, field: Field) -> Vector{
        let mut total_force = Vector::new();
        for force in applied_forces{
            total_force += force;
        }
        total_force.mul(1. / self.mass);
        return total_force;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector{
    pub x: f32,
    pub y: f32,
}

impl Vector{
    pub fn new() -> Self{
        return Self { x: 0., y: 0. }
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_mut_x(&mut self) -> &mut f32 {
        &mut self.x
    }

    fn get_mut_y(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn get_magnitude(&self) -> f32{
        (self.get_x().powf(2.) + self.get_y().powf(2.)).sqrt()
    }
    fn normalize(&mut self){
        let mag = self.get_magnitude();
        self.mul(1. / mag);
    }
    fn add(&mut self, x: f32, y: f32){
        let mut_x = self.get_mut_x();
        *mut_x += x;
        let mut_y = self.get_mut_y();
        *mut_y += y;
    }
    fn mul(&mut self, scalar: f32){
        *self.get_mut_x() *= scalar;
        *self.get_mut_y() *= scalar;
    }
    fn add_vector(&mut self, other: Vector){
        self.add(other.get_x(), other.get_y())
    }
    fn sub_vector(&mut self, other: Vector){
        self.add(-1. * other.get_x(), -1. * other.get_y())
    }
}

impl AddAssign for Vector{
    fn add_assign(&mut self, rhs: Vector) {
        self.add_vector(rhs)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.mul(rhs);
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_vector(rhs);
    }
}

pub struct Field{

}