//! `fast-find` finds bytes fast.
//!
//! Fast-find uses SIMD instructions to make searching small byte arrays fast, or
//! falls back to linear search when not possible.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "arch/x64.rs"]
mod arch;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[path = "fallback.rs"]
mod arch;

mod fallback;


pub trait FastFind {
    /// Returns the index of the first element in the haystack that matches needle, or an out of bounds index if not found
    fn fast_find(&self, needle: u8) -> usize;
}

impl FastFind for [u8; 4] {
    #[inline]
    fn fast_find(&self, needle: u8) -> usize {
        arch::find_in_4(needle, self)
    }
}

impl FastFind for [u8; 16] {
    fn fast_find(&self, needle: u8) -> usize {
        arch::find_in_16(needle, self)
    }
}

impl FastFind for [u8; 32] {
    #[inline]
    fn fast_find(&self, needle: u8) -> usize {
        arch::find_in_32(needle, self)
    }
}

#[cfg(test)]
mod tests {
    use super::FastFind;

    #[test]
    fn test_find_4() {
        let mut array = [0; 4];

        assert_eq!(array.fast_find(4), 4);

        array[1] = 4;
        array[3] = 10;

        assert_eq!(array.fast_find(4), 1);
        assert_eq!(array.fast_find(5), 4);
        assert_eq!(array.fast_find(10), 3);
    }

    #[test]
    fn test_find_16() {
        let mut array = [0; 16];

        assert_eq!(array.fast_find(4), 32);

        array[4] = 4;
        array[6] = 10;

        assert_eq!(array.fast_find(4), 4);
        assert_eq!(array.fast_find(5), 32);
        assert_eq!(array.fast_find(10), 6);
    }

    #[test]
    fn test_find_32() {
        let mut array = [0; 32];

        assert_eq!(array.fast_find(4), 32);

        array[4] = 4;
        array[6] = 10;
        array[20] = 5;

        assert_eq!(array.fast_find(4), 4);
        assert_eq!(array.fast_find(5), 20);
        assert_eq!(array.fast_find(10), 6);
    }
}
