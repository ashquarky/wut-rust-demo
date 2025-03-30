use core::cmp::min;
use core::ffi::CStr;

#[derive(Debug, PartialEq)]
pub(crate) struct StackCString<const N: usize>([u8; N]);

impl<const N: usize> StackCString<N> {
    /* the most "logic error"-coded shit imaginable */
    pub(crate) fn from(s: &str) -> Self {
        let mut buffer = [0u8; N];
        let len = min(s.len(), N - 1); // -1 for null
        for (dst, c) in buffer[..len].iter_mut().zip(s.bytes()) {
            *dst = c;
        }

        Self(buffer)
    }

    pub(crate) fn c_str(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.0).unwrap_or(c"string conversion error")
    }
}