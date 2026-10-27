pub mod _shared;
pub mod checks;
pub mod denormalize;
pub mod file_dialect;
pub mod inspect;
pub mod normalize;
pub mod query;

#[cfg(test)]
#[path = "-test/mod.rs"]
mod _test;
