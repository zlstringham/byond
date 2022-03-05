cfg_if::cfg_if! {
    if #[cfg(all(
        target_feature = "sse2",
        any(target_arch = "x86", target_arch = "x86_64")
    ))] {
        mod pclmulqdq;
        pub use pclmulqdq::State;
    } else {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum State {}
        impl State {
            pub fn new(_: u32) -> Option<Self> {
                None
            }

            pub fn update(&mut self, _buf: &[u8]) {
                unimplemented!()
            }

            pub fn finalize(self) -> u32 {
                unimplemented!()
            }

            pub fn reset(&mut self) {
                unimplemented!()
            }

            pub fn combine(&mut self, _other: u32, _amount: u64) {
                unimplemented!()
            }
        }
    }
}
