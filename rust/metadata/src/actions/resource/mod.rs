pub mod data;
pub mod denormalize;
pub mod from_frictionless;
pub mod general;
pub mod infer;
pub mod normalize;
pub mod to_frictionless;

#[cfg(test)]
#[path = "-test/mod.rs"]
mod _test;
