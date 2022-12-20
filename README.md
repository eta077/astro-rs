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
* Deserialize as lazily as possible
* Balance 'tight' (<https://www.ecorax.net/tightness>) types and adherance to the FITS API with graceful handling of deviation

# Testing
Test assets are from the following sources:
* <https://esahubble.org/projects/fits_liberator/datasets_archives>
* <https://cxc.harvard.edu/cda/>
