pub mod data;
pub mod denormalize;
pub mod general;
pub mod infer;
pub mod normalize;

#[cfg(test)]
#[path = "-test/mod.rs"]
mod _test;
