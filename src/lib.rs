#![deny(missing_docs)]
#![deny(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "iers")]
#[cfg_attr(docsrs, doc(cfg(feature = "iers")))]
pub mod iers;

#[cfg(feature = "coordinates")]
#[cfg_attr(docsrs, doc(cfg(feature = "coordinates")))]
pub mod coordinates;

#[cfg(feature = "cosmology")]
#[cfg_attr(docsrs, doc(cfg(feature = "cosmology")))]
pub mod cosmology;

#[cfg(feature = "fits")]
#[cfg_attr(docsrs, doc(cfg(feature = "fits")))]
pub mod fits;
