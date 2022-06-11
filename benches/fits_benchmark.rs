use std::fs::File;
use std::io::BufReader;

use astro_rs::fits::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_get_header(c: &mut Criterion) {
    c.bench_function("get header from fits file", |b| {
        b.iter(|| {
            let fits_file = black_box(File::open("assets/benchmarks/many_hdu.fits").unwrap());
            let fits_file_reader = BufReader::new(fits_file);

            let mut hdu_list = HduList::new(fits_file_reader);
            hdu_list.first_mut().unwrap().header.clone()
        })
    });
}

fn bench_get_header_20(c: &mut Criterion) {
    c.bench_function("get 20th header from fits file", |b| {
        b.iter(|| {
            let fits_file = black_box(File::open("assets/benchmarks/many_hdu.fits").unwrap());
            let fits_file_reader = BufReader::new(fits_file);

            let mut hdu_list = HduList::new(fits_file_reader);
            hdu_list.get_by_index(20).unwrap().header.clone()
        })
    });
}

criterion_group!(benches, bench_get_header, bench_get_header_20);
criterion_main!(benches);
