#![deny(missing_docs)]
#![deny(clippy::all)]
//! This library provides utilities to interact with astronomical data.

#[cfg(feature = "coordinates")]
pub mod coordinates;
pub mod fits;
