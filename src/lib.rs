#![deny(missing_docs)]
#![deny(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! This library provides utilities to interact with astronomical data.

#[cfg(feature = "coordinates")]
#[cfg_attr(docsrs, doc(cfg(feature = "coordinates")))]
pub mod coordinates;
pub mod fits;
