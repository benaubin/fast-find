//! `fast-find` finds bytes fast.
//!
//! Fast-find uses SIMD instructions to make searching small byte arrays fast.
//! It falls back to linear search when these instructions are not available.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "arch/x64.rs"]
mod arch;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[path = "fallback.rs"]
mod arch;

mod fallback;

pub trait FastFind {
    /// Returns the index of the first element in the haystack that matches
    /// `needle`. If the index is larger than `len`, it will return None.
    fn fast_find(&self, needle: u8, len: usize) -> Option<usize>;
}

macro_rules! impl_fast_find {
    ($($bytes:literal),*) => {
        paste::paste! {
            $(
                impl FastFind for [u8; $bytes] {
                    #[inline]
                    fn fast_find(&self, needle: u8, len: usize) -> Option<usize> {
                        arch::[<find_in_ $bytes>](needle, self, len)
                    }
                }
            )*
        }
    };
}

impl_fast_find!(4, 16, 32);

#[cfg(test)]
mod tests {
    use super::FastFind;

    #[test]
    fn test_find_4() {
        let mut array = [0; 4];

        assert_eq!(array.fast_find(4, 4), None);

        array[1] = 4;
        array[3] = 10;

        assert_eq!(array.fast_find(4, 4), Some(1));
        assert_eq!(array.fast_find(5, 4), None);
        assert_eq!(array.fast_find(10, 4), Some(3));

        assert_eq!(array.fast_find(4, 2), Some(1));
        assert_eq!(array.fast_find(5, 2), None);
        assert_eq!(array.fast_find(10, 2), None);
    }

    #[test]
    fn test_find_16() {
        let mut array = [0; 16];

        assert_eq!(array.fast_find(4, 16), None);

        array[4] = 4;
        array[6] = 10;

        assert_eq!(array.fast_find(4, 16), Some(4));
        assert_eq!(array.fast_find(5, 16), None);
        assert_eq!(array.fast_find(10, 16), Some(6));

        assert_eq!(array.fast_find(4, 5), Some(4));
        assert_eq!(array.fast_find(5, 5), None);
        assert_eq!(array.fast_find(10, 5), None);
    }

    #[test]
    fn test_find_32() {
        let mut array = [0; 32];

        assert_eq!(array.fast_find(4, 16), None);

        array[4] = 4;
        array[6] = 10;
        array[20] = 5;

        assert_eq!(array.fast_find(4, 32), Some(4));
        assert_eq!(array.fast_find(5, 32), Some(20));
        assert_eq!(array.fast_find(10, 32), Some(6));

        assert_eq!(array.fast_find(4, 5), Some(4));
        assert_eq!(array.fast_find(5, 5), None);
        assert_eq!(array.fast_find(10, 5), None);
    }
}
