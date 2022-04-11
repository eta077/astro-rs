use std::fs::File;
use std::io::{BufReader, Read};

use astro_rs::fits::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_get_header(c: &mut Criterion) {
    c.bench_function("get header from fits file", |b| {
        b.iter(|| {
            let fits_file = black_box(File::open("assets/benchmarks/many_hdu.fits").unwrap());
            let mut fits_file_reader = BufReader::new(fits_file);
            let mut fits_bytes = Vec::new();
            fits_file_reader.read_to_end(&mut fits_bytes).unwrap();

            let hdu_list = HduList::from_bytes(fits_bytes.clone()).unwrap();
            hdu_list.hdus.first().unwrap().header.clone()
        })
    });
}

criterion_group!(benches, bench_get_header);
criterion_main!(benches);
