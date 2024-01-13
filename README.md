![GitHub Build Status (branch)](https://img.shields.io/github/actions/workflow/status/eta077/astro-rs/build.yml?branch=release) 
![GitHub Test Status (branch)](https://img.shields.io/github/actions/workflow/status/eta077/astro-rs/test.yml?branch=release&label=test) 
[![codecov](https://codecov.io/gh/eta077/astro-rs/branch/release/graph/badge.svg)](https://codecov.io/gh/eta077/astro-rs) 
[![docs.rs](https://img.shields.io/docsrs/astro-rs)](https://docs.rs/astro-rs/latest/astro_rs/)

# astro-rs
This library provides utilities to interact with astronomical data.

Inspired by Astropy (<http://astropy.org> / <https://github.com/astropy/astropy>)

# Goals
## General goals
* Achieve feature compatibility with the Astropy library
* Equal or surpass the Astropy benchmarks

## Technical goals
* Use pure Rust as opposed to wrapping other libraries
* Deserialize as lazily as possible
* Balance 'tight' (<https://www.ecorax.net/tightness>) types and adherance to APIs with graceful handling of deviation

# Testing
Test assets are from the following sources:
* <https://esahubble.org/projects/fits_liberator/datasets_archives>
* <https://cxc.harvard.edu/cda/>

# Licensing
* Original code is licensed under the MIT license
* `astropy` is licensed under BSD-3-Clause
* `hifitime` is licensed under Apache-2.0

# MSRV
This crate's Minimum Supported Rust Version is `1.68.2`.
