const COMBINE_TABLE: [u32; 32] = [
    0x00000100, 0x00010000, 0x000000af, 0x00004455, 0x10101111, 0xae01ae01, 0x1bd81099, 0x87f2f581,
    0x9dd170d4, 0x5bbedfd6, 0x27afa5be, 0xf0db1b29, 0x2264a683, 0xfaa007ab, 0x0a402c54, 0x2d26e110,
    0xc99cb412, 0x5e545a7b, 0x6dc24493, 0x7dae76c1, 0xd7a20af5, 0x978e3ee2, 0x629ca6e1, 0x0a831f49,
    0x2802d252, 0xc6c412e6, 0x39697bab, 0x2d222274, 0x9999aaf2, 0x00000002, 0x00000004, 0x00000010,
];

#[inline(always)]
pub(crate) fn combine(mut crc1: u32, crc2: u32, len2: u64) -> u32 {
    crc1 ^= 0xffffffff;
    multmodp(x8nmodp(len2), crc1) ^ crc2
}

#[inline(always)]
fn multmodp(mut a: u32, mut b: u32) -> u32 {
    let mut prod = 0;
    loop {
        if a & 1 != 0 {
            prod ^= b;
            if a == 1 {
                break;
            }
        }
        a >>= 1;
        b = if b & 0x80000000 != 0 {
            (b << 1) ^ 0xaf
        } else {
            b << 1
        };
    }
    prod
}

#[inline(always)]
fn x8nmodp(mut n: u64) -> u32 {
    let mut xp = 1;
    let mut k = 0;
    while n != 0 {
        if n & 1 != 0 {
            xp = multmodp(COMBINE_TABLE[k], xp);
        }
        n >>= 1;
        k += 1;
        if k == 32 {
            k = 0;
        }
    }
    xp
}
