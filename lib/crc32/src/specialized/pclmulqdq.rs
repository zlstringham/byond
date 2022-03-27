#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    state: u32,
}

impl State {
    #[cfg(not(feature = "std"))]
    pub fn new(state: u32) -> Option<Self> {
        if cfg!(all(
            target_feature = "sse4.1"
            target_feature = "pclmulqdq",
        )) {
            Some(Self { state })
        } else {
            None
        }
    }

    #[cfg(feature = "std")]
    pub fn new(state: u32) -> Option<Self> {
        if is_x86_feature_detected!("pclmulqdq") && is_x86_feature_detected!("sse4.1") {
            Some(Self { state })
        } else {
            None
        }
    }

    pub fn update(&mut self, buf: &[u8]) {
        self.state = unsafe { calculate(self.state, buf) }
    }

    pub fn as_u32(&self) -> u32 {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = crate::DEFAULT_CRC32;
    }
}

const RK01: u64 = 0x0029_5f23_0000_0000;
const RK02: u64 = 0xfafa_5179_0000_0000;
const RK03: u64 = 0x5cd8_6bb5_0000_0000;
const RK04: u64 = 0xaf6f_37a3_0000_0000;
const RK05: u64 = 0x0029_5f23_0000_0000;
const RK06: u64 = 0x0000_4455_0000_0000;
const RK07: u64 = 0x0000_0001_0000_00af;
const RK08: u64 = 0x0000_0001_0000_00af;
const RK09: u64 = 0x9bd5_7b5d_0000_0000;
const RK10: u64 = 0xb7a4_d764_0000_0000;
const RK11: u64 = 0x1ae0_0042_0000_0000;
const RK12: u64 = 0xe772_0be6_0000_0000;
const RK13: u64 = 0x9c7f_c8fe_0000_0000;
const RK14: u64 = 0x3885_faf8_0000_0000;
const RK15: u64 = 0xb477_ad71_0000_0000;
const RK16: u64 = 0x0ac2_ae3d_0000_0000;
const RK17: u64 = 0x5eae_9dbe_0000_0000;
const RK18: u64 = 0x784a_4838_0000_0000;
const RK19: u64 = 0x7d21_bf20_0000_0000;
const RK20: u64 = 0xfaeb_d3d3_0000_0000;

#[target_feature(enable = "pclmulqdq", enable = "sse4.1")]
pub unsafe fn calculate(crc: u32, mut data: &[u8]) -> u32 {
    if data.len() < 16 * 8 * 2 {
        // This could be handled in intrinsics, but this seems fine for now.
        return crate::baseline::slice_by_16(crc, data);
    }

    let crc = _mm_set_epi32(crc as i32, 0x0000, 0x0000, 0x0000);
    // Shuffle mask for byte-swapping 16 bytes.
    let smask = _mm_set_epi8(
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    );

    // Load initial 128B of data and XOR in the initial CRC.
    let mut x7 = get(&mut data, smask);
    let mut x6 = get(&mut data, smask);
    let mut x5 = get(&mut data, smask);
    let mut x4 = get(&mut data, smask);
    let mut x3 = get(&mut data, smask);
    let mut x2 = get(&mut data, smask);
    let mut x1 = get(&mut data, smask);
    let mut x0 = get(&mut data, smask);
    x7 = _mm_xor_si128(x7, crc);

    let k3k4 = _mm_set_epi64x(RK04 as i64, RK03 as i64);
    while data.len() >= 128 {
        x7 = reduce128(x7, get(&mut data, smask), k3k4);
        x6 = reduce128(x6, get(&mut data, smask), k3k4);
        x5 = reduce128(x5, get(&mut data, smask), k3k4);
        x4 = reduce128(x4, get(&mut data, smask), k3k4);
        x3 = reduce128(x3, get(&mut data, smask), k3k4);
        x2 = reduce128(x2, get(&mut data, smask), k3k4);
        x1 = reduce128(x1, get(&mut data, smask), k3k4);
        x0 = reduce128(x0, get(&mut data, smask), k3k4);
    }

    let k1k2 = _mm_set_epi64x(RK02 as i64, RK01 as i64);
    let mut x = reduce128(x7, x0, _mm_set_epi64x(RK10 as i64, RK09 as i64));
    x = reduce128(x6, x, _mm_set_epi64x(RK12 as i64, RK11 as i64));
    x = reduce128(x5, x, _mm_set_epi64x(RK14 as i64, RK13 as i64));
    x = reduce128(x4, x, _mm_set_epi64x(RK16 as i64, RK15 as i64));
    x = reduce128(x3, x, _mm_set_epi64x(RK18 as i64, RK17 as i64));
    x = reduce128(x2, x, _mm_set_epi64x(RK20 as i64, RK19 as i64));
    x = reduce128(x1, x, k1k2);

    while data.len() >= 16 {
        x = reduce128(x, get(&mut data, smask), k1k2);
    }

    // Reduce 128b to 64b.
    let k5k6 = _mm_set_epi64x(RK06 as i64, RK05 as i64);
    x = _mm_xor_si128(_mm_clmulepi64_si128(x, k5k6, 0x01), _mm_slli_si128(x, 8));
    x = _mm_xor_si128(
        _mm_clmulepi64_si128(_mm_srli_si128(x, 12), k5k6, 0x10),
        _mm_and_si128(x, _mm_set_epi32(0, !0, !0, !0)),
    );

    // Barrett reduction, 64b to 32b.
    let k7k8 = _mm_set_epi64x(RK08 as i64, RK07 as i64);
    let t1 = _mm_slli_si128(_mm_clmulepi64_si128(x, k7k8, 0x01), 4);
    let t2 = _mm_slli_si128(_mm_clmulepi64_si128(t1, k7k8, 0x11), 4);
    let crc = _mm_extract_epi32(_mm_xor_si128(x, t2), 1) as u32;

    if data.is_empty() {
        crc
    } else {
        // We could use intrinsics for the remaining data, but this seems fine for now.
        // Less than 16B remaining, so slice-by-1 instead of slice-by-16.
        crate::baseline::slice_by_1(crc, data)
    }
}

#[inline(always)]
unsafe fn reduce128(a: __m128i, b: __m128i, keys: __m128i) -> __m128i {
    let t1 = _mm_clmulepi64_si128(a, keys, 0x00);
    let t2 = _mm_clmulepi64_si128(a, keys, 0x11);
    _mm_xor_si128(_mm_xor_si128(b, t1), t2)
}

#[inline(always)]
unsafe fn get(data: &mut &[u8], smask: __m128i) -> __m128i {
    let r = _mm_shuffle_epi8(_mm_loadu_si128(data.as_ptr() as *const __m128i), smask);
    *data = &data[16..];
    r
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn check_against_baseline(init: u32, chunks: Vec<(Vec<u8>, usize)>) -> bool {
        let mut baseline = crate::baseline::State::new(init);
        let mut pclmulqdq = super::State::new(init).expect("not supported");
        for (chunk, mut offset) in chunks {
            // simulate random alignments by offsetting the slice by up to 15 bytes
            offset &= 0xF;
            if chunk.len() <= offset {
                baseline.update(&chunk);
                pclmulqdq.update(&chunk);
            } else {
                baseline.update(&chunk[offset..]);
                pclmulqdq.update(&chunk[offset..]);
            }
        }
        pclmulqdq.as_u32() == baseline.as_u32()
    }
}
