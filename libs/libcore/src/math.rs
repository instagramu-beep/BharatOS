//! BharatOS libcore math — portable SIMD-aware scalar + vector math
#![no_std]
#![allow(unused)]

pub use core::f32::consts::{PI, TAU, E, FRAC_PI_2, FRAC_PI_4, LN_2, LN_10};
pub use core::f64::consts::{PI as PI64, TAU as TAU64};

pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;

#[inline(always)]
pub fn sin(x: f32) -> f32 { x.sin() }
#[inline(always)]
pub fn cos(x: f32) -> f32 { x.cos() }
#[inline(always)]
pub fn tan(x: f32) -> f32 { x.tan() }
#[inline(always)]
pub fn asin(x: f32) -> f32 { x.asin() }
#[inline(always)]
pub fn acos(x: f32) -> f32 { x.acos() }
#[inline(always)]
pub fn atan(x: f32) -> f32 { x.atan() }
#[inline(always)]
pub fn atan2(y: f32, x: f32) -> f32 { y.atan2(x) }
#[inline(always)]
pub fn sqrt(x: f32) -> f32 { x.sqrt() }
#[inline(always)]
pub fn pow(x: f32, y: f32) -> f32 { x.powf(y) }
#[inline(always)]
pub fn exp(x: f32) -> f32 { x.exp() }
#[inline(always)]
pub fn ln(x: f32) -> f32 { x.ln() }
#[inline(always)]
pub fn log2(x: f32) -> f32 { x.log2() }
#[inline(always)]
pub fn log10(x: f32) -> f32 { x.log10() }
#[inline(always)]
pub fn abs(x: f32) -> f32 { x.abs() }
#[inline(always)]
pub fn floor(x: f32) -> f32 { x.floor() }
#[inline(always)]
pub fn ceil(x: f32) -> f32 { x.ceil() }
#[inline(always)]
pub fn round(x: f32) -> f32 { x.round() }
#[inline(always)]
pub fn fract(x: f32) -> f32 { x.fract() }
#[inline(always)]
pub fn copysign(x: f32, y: f32) -> f32 { x.copysign(y) }
#[inline(always)]
pub fn fmod(x: f32, y: f32) -> f32 { x % y }
#[inline(always)]
pub fn remainder(x: f32, y: f32) -> f32 { ((x % y) + y) % y }
#[inline(always)]
pub fn frexp(x: f32) -> (f32, i32) { x.frexp() }
#[inline(always)]
pub fn ldexp(x: f32, exp: i32) -> f32 { x.ldexp(exp) }

pub fn clamp(x: f32, min: f32, max: f32) -> f32 { x.clamp(min, max) }
pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
pub fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = sqrt(dot(v, v));
    if len == 0.0 { [0.0; 3] } else { [v[0]/len, v[1]/len, v[2]/len] }
}
pub fn length(v: [f32; 3]) -> f32 { sqrt(dot(v, v)) }
pub fn distance(a: [f32; 3], b: [f32; 3]) -> f32 { length([b[0]-a[0], b[1]-a[1], b[2]-a[2]]) }
pub fn reflect(normal: [f32; 3], incident: [f32; 3]) -> [f32; 3] {
    let d = 2.0 * dot(normal, incident);
    [incident[0] - d*normal[0], incident[1] - d*normal[1], incident[2] - d*normal[2]]
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn dot(self, o: Self) -> f32 { self.x*o.x + self.y*o.y }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self { let l=self.length(); Self { x:self.x/l, y:self.y/l } }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn dot(self, o: Self) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
    pub fn cross(self, o: Self) -> Self {
        Self { x: self.y*o.z-self.z*o.y, y: self.z*o.x-self.x*o.z, z: self.x*o.y-self.y*o.x }
    }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self { let l=self.length(); Self { x:self.x/l, y:self.y/l, z:self.z/l } }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Mat4x4 {
    pub m: [f32; 16],
}

impl Mat4x4 {
    pub const fn identity() -> Self {
        Self { m: [
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0,
        ]}
    }
}
