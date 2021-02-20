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

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
#[inline]
pub fn find_in_4(needle: u8, haystack: &[u8; 4], len: usize) -> Option<usize> {
    arch::find_in_4(needle, haystack, len)
}

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
#[inline]
pub fn find_in_16(needle: u8, haystack: &[u8; 16], len: usize) -> Option<usize> {
    arch::find_in_16(needle, haystack, len)
}

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
#[inline]
pub fn find_in_32(needle: u8, haystack: &[u8; 32], len: usize) -> Option<usize> {
    arch::find_in_32(needle, haystack, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_4() {
        let mut array = [0; 4];

        assert_eq!(find_in_4(4, &array, 4), None);

        array[1] = 4;
        array[3] = 10;

        assert_eq!(find_in_4(4, &array, 4), Some(1));
        assert_eq!(find_in_4(5, &array, 4), None);
        assert_eq!(find_in_4(10, &array, 4), Some(3));

        assert_eq!(find_in_4(4, &array, 2), Some(1));
        assert_eq!(find_in_4(5, &array, 2), None);
        assert_eq!(find_in_4(10, &array, 2), None);
    }

    #[test]
    fn test_find_16() {
        let mut array = [0; 16];

        assert_eq!(find_in_16(4, &array, 16), None);

        array[4] = 4;
        array[6] = 10;

        assert_eq!(find_in_16(4, &array, 16), Some(4));
        assert_eq!(find_in_16(5, &array, 16), None);
        assert_eq!(find_in_16(10, &array, 16), Some(6));

        assert_eq!(find_in_16(4, &array, 5), Some(4));
        assert_eq!(find_in_16(5, &array, 5), None);
        assert_eq!(find_in_16(10, &array, 5), None);
    }

    #[test]
    fn test_find_32() {
        let mut array = [0; 32];

        assert_eq!(find_in_32(4, &array, 16), None);

        array[4] = 4;
        array[6] = 10;
        array[20] = 5;

        assert_eq!(find_in_32(4, &array, 32), Some(4));
        assert_eq!(find_in_32(5, &array, 32), Some(20));
        assert_eq!(find_in_32(10, &array, 32), Some(6));

        assert_eq!(find_in_32(4, &array, 5), Some(4));
        assert_eq!(find_in_32(5, &array, 5), None);
        assert_eq!(find_in_32(10, &array, 5), None);
    }
}
