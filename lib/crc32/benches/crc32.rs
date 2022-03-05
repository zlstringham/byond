use byond_crc32::{baseline, specialized};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn crc32_4kib_baseline(c: &mut Criterion) {
    c.bench_function("CRC32/BYOND 4KiB Baseline", |b| {
        let v = vec![13u8; 4 * 1024];
        b.iter(|| {
            let mut crc = baseline::State::new(0xffffffff);
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn crc32_4mib_baseline(c: &mut Criterion) {
    c.bench_function("CRC32/BYOND 4MiB Baseline", |b| {
        let v = vec![13u8; 4 * 1024 * 1024];
        b.iter(|| {
            let mut crc = baseline::State::new(0xffffffff);
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn crc32_4gib_baseline(c: &mut Criterion) {
    c.bench_function("CRC32/BYOND 4GiB Baseline", |b| {
        let v = vec![13u8; 4 * 1024 * 1024 * 1024];
        b.iter(|| {
            let mut crc = baseline::State::new(0xffffffff);
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn crc32_4kib_specialized(c: &mut Criterion) {
    if specialized::State::new(0xffffffff).is_none() {
        return;
    }
    c.bench_function("CRC32/BYOND 4KiB Specialized", |b| {
        let v = vec![13u8; 4 * 1024];
        b.iter(|| {
            let mut crc = specialized::State::new(0xffffffff).unwrap();
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn crc32_4mib_specialized(c: &mut Criterion) {
    if specialized::State::new(0xffffffff).is_none() {
        return;
    }
    c.bench_function("CRC32/BYOND 4MiB Specialized", |b| {
        let v = vec![13u8; 4 * 1024 * 1024];
        b.iter(|| {
            let mut crc = specialized::State::new(0xffffffff).unwrap();
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn crc32_4gib_specialized(c: &mut Criterion) {
    if specialized::State::new(0xffffffff).is_none() {
        return;
    }
    c.bench_function("CRC32/BYOND 4GiB Specialized", |b| {
        let v = vec![13u8; 4 * 1024 * 1024 * 1024];
        b.iter(|| {
            let mut crc = specialized::State::new(0xffffffff).unwrap();
            crc.update(black_box(v.as_slice()));
            crc.as_u32()
        });
    });
}

fn naive_crc32(crc: u32, bytes: &[u8]) -> u32 {
    bytes.iter().fold(crc, |crc, &byte| {
        (crc << 8) ^ BYTE_TABLE[(crc >> 24) as usize ^ byte as usize]
    })
}

fn naive_crc32_4kib(c: &mut Criterion) {
    c.bench_function("CRC32/BYOND 4KiB Naive", |b| {
        let v = vec![13u8; 4 * 1024];
        b.iter(|| naive_crc32(0xffffff, v.as_slice()));
    });
}

fn naive_crc32_4mib(c: &mut Criterion) {
    c.bench_function("CRC32/BYOND 4MiB Naive", |b| {
        let v = vec![13u8; 4 * 1024 * 1024];
        b.iter(|| naive_crc32(0xffffff, v.as_slice()));
    });
}

fn naive_crc32_4gib(c: &mut Criterion) {
    c // This benchmark takes forever otherwise.
        .bench_function("CRC32/BYOND 4GiB Naive", |b| {
            let v = vec![13u8; 4 * 1024 * 1024 * 1024];
            b.iter(|| naive_crc32(0xffffff, v.as_slice()));
        });
}

criterion_group! {
    name = naive_benches;
    config = Criterion::default().sample_size(10);
    targets = naive_crc32_4kib, naive_crc32_4mib, naive_crc32_4gib
}
criterion_group! {
    name = baseline_benches;
    config = Criterion::default().sample_size(10);
    targets = crc32_4kib_baseline, crc32_4mib_baseline, crc32_4gib_baseline
}
criterion_group! {
    name = specialized_benches;
    config = Criterion::default().sample_size(10);
    targets = crc32_4kib_specialized, crc32_4mib_specialized, crc32_4gib_specialized
}
criterion_main!(naive_benches, baseline_benches, specialized_benches);

const BYTE_TABLE: [u32; 256] = [
    0x0000, 0x00af, 0x015e, 0x01f1, 0x02bc, 0x0213, 0x03e2, 0x034d, 0x0578, 0x05d7, 0x0426, 0x0489,
    0x07c4, 0x076b, 0x069a, 0x0635, 0x0af0, 0x0a5f, 0x0bae, 0x0b01, 0x084c, 0x08e3, 0x0912, 0x09bd,
    0x0f88, 0x0f27, 0x0ed6, 0x0e79, 0x0d34, 0x0d9b, 0x0c6a, 0x0cc5, 0x15e0, 0x154f, 0x14be, 0x1411,
    0x175c, 0x17f3, 0x1602, 0x16ad, 0x1098, 0x1037, 0x11c6, 0x1169, 0x1224, 0x128b, 0x137a, 0x13d5,
    0x1f10, 0x1fbf, 0x1e4e, 0x1ee1, 0x1dac, 0x1d03, 0x1cf2, 0x1c5d, 0x1a68, 0x1ac7, 0x1b36, 0x1b99,
    0x18d4, 0x187b, 0x198a, 0x1925, 0x2bc0, 0x2b6f, 0x2a9e, 0x2a31, 0x297c, 0x29d3, 0x2822, 0x288d,
    0x2eb8, 0x2e17, 0x2fe6, 0x2f49, 0x2c04, 0x2cab, 0x2d5a, 0x2df5, 0x2130, 0x219f, 0x206e, 0x20c1,
    0x238c, 0x2323, 0x22d2, 0x227d, 0x2448, 0x24e7, 0x2516, 0x25b9, 0x26f4, 0x265b, 0x27aa, 0x2705,
    0x3e20, 0x3e8f, 0x3f7e, 0x3fd1, 0x3c9c, 0x3c33, 0x3dc2, 0x3d6d, 0x3b58, 0x3bf7, 0x3a06, 0x3aa9,
    0x39e4, 0x394b, 0x38ba, 0x3815, 0x34d0, 0x347f, 0x358e, 0x3521, 0x366c, 0x36c3, 0x3732, 0x379d,
    0x31a8, 0x3107, 0x30f6, 0x3059, 0x3314, 0x33bb, 0x324a, 0x32e5, 0x5780, 0x572f, 0x56de, 0x5671,
    0x553c, 0x5593, 0x5462, 0x54cd, 0x52f8, 0x5257, 0x53a6, 0x5309, 0x5044, 0x50eb, 0x511a, 0x51b5,
    0x5d70, 0x5ddf, 0x5c2e, 0x5c81, 0x5fcc, 0x5f63, 0x5e92, 0x5e3d, 0x5808, 0x58a7, 0x5956, 0x59f9,
    0x5ab4, 0x5a1b, 0x5bea, 0x5b45, 0x4260, 0x42cf, 0x433e, 0x4391, 0x40dc, 0x4073, 0x4182, 0x412d,
    0x4718, 0x47b7, 0x4646, 0x46e9, 0x45a4, 0x450b, 0x44fa, 0x4455, 0x4890, 0x483f, 0x49ce, 0x4961,
    0x4a2c, 0x4a83, 0x4b72, 0x4bdd, 0x4de8, 0x4d47, 0x4cb6, 0x4c19, 0x4f54, 0x4ffb, 0x4e0a, 0x4ea5,
    0x7c40, 0x7cef, 0x7d1e, 0x7db1, 0x7efc, 0x7e53, 0x7fa2, 0x7f0d, 0x7938, 0x7997, 0x7866, 0x78c9,
    0x7b84, 0x7b2b, 0x7ada, 0x7a75, 0x76b0, 0x761f, 0x77ee, 0x7741, 0x740c, 0x74a3, 0x7552, 0x75fd,
    0x73c8, 0x7367, 0x7296, 0x7239, 0x7174, 0x71db, 0x702a, 0x7085, 0x69a0, 0x690f, 0x68fe, 0x6851,
    0x6b1c, 0x6bb3, 0x6a42, 0x6aed, 0x6cd8, 0x6c77, 0x6d86, 0x6d29, 0x6e64, 0x6ecb, 0x6f3a, 0x6f95,
    0x6350, 0x63ff, 0x620e, 0x62a1, 0x61ec, 0x6143, 0x60b2, 0x601d, 0x6628, 0x6687, 0x6776, 0x67d9,
    0x6494, 0x643b, 0x65ca, 0x6565,
];
