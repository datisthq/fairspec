pub mod error;
pub mod metadata;
pub mod potential_dialects;
pub mod sample;
pub mod score;
#[allow(clippy::module_inception)]
pub mod sniffer;
pub mod table;
pub mod uniformity;

#[cfg(test)]
#[path = "-test/mod.rs"]
mod _test;
