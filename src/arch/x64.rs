#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem::transmute;

use std::is_x86_feature_detected;

use crate::fallback;

pub use crate::fallback::find_in_4;

#[inline]
pub fn find_in_16(needle: u8, haystack: &[u8; 16], len: usize) -> Option<usize> {
    if is_x86_feature_detected!("sse2") {
        let bitfield = unsafe { sse2_eq_16(needle, haystack.as_ptr()) };
        find_in_bitfield(bitfield, len)
    } else {
        fallback::find_in_16(needle, haystack, len)
    }
}

#[inline]
pub fn find_in_32(needle: u8, haystack: &[u8; 32], len: usize) -> Option<usize> {
    let bitfield = if is_x86_feature_detected!("avx2") {
        unsafe { avx2_eq_32(needle, haystack.as_ptr()) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { sse2_eq_32(needle, haystack.as_ptr()) }
    } else {
        return fallback::find_in_32(needle, haystack, len);
    };

    find_in_bitfield(bitfield, len)
}

/// Finds the index of the last 1 in the bitfield, up-to len
#[inline]
fn find_in_bitfield(bitfield: i32, len: usize) -> Option<usize> {
    let idx = bitfield.trailing_zeros() as usize;
    if len > idx {
        Some(idx)
    } else {
        None
    }
}

/// Returns a bitfield, where all elements in haystack that are equal to needle = 1, else 0
///
/// Safety: requires sse2, 16 byte haystack
#[inline]
unsafe fn sse2_eq_16(needle: u8, haystack: *const u8) -> i32 {
    let haystack = _mm_loadu_si128(haystack as *const __m128i); // SSE2
    let needle_vec = _mm_set1_epi8(transmute::<u8, i8>(needle)); // SSE2
    let eq = _mm_cmpeq_epi8(needle_vec, haystack); // SSE2
    _mm_movemask_epi8(eq) // SSE2
}

/// Returns a bitfield, where all elements in haystack that are equal to needle = 1, else 0
///
/// Safety: requires sse2, 32 byte haystack
#[inline]
unsafe fn sse2_eq_32(needle: u8, haystack: *const u8) -> i32 {
    sse2_eq_16(needle, haystack) | (sse2_eq_16(needle, haystack.offset(16)) << 16)
}

/// Returns a bitfield, where all elements in haystack that are equal to needle = 1, else 0
///
/// Safety: requires avx2, 32 byte haystack
#[inline]
unsafe fn avx2_eq_32(needle: u8, haystack: *const u8) -> i32 {
    let haystack = _mm256_loadu_si256(haystack as *const __m256i);
    let needle_vec = _mm256_set1_epi8(transmute::<u8, i8>(needle));
    let eq = _mm256_cmpeq_epi8(needle_vec, haystack);
    _mm256_movemask_epi8(eq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx_find_16() {
        fn test_all(needle: u8, haystack: &[u8; 16], len: usize, expected: Option<usize>) {
            assert_eq!(
                find_in_bitfield(unsafe { sse2_eq_16(needle, haystack.as_ptr()) }, len),
                expected
            );
            assert_eq!(find_in_16(needle, haystack, len), expected);
        }

        let mut array = [0; 16];

        test_all(4, &array, 16, None);

        array[4] = 4;
        array[6] = 10;

        test_all(4, &array, 16, Some(4));
        test_all(5, &array, 16, None);
        test_all(10, &array, 16, Some(6));

        test_all(4, &array, 5, Some(4));
        test_all(5, &array, 5, None);
        test_all(10, &array, 5, None);
    }

    #[test]
    fn test_avx_find_32() {
        fn test_all(needle: u8, haystack: &[u8; 32], len: usize, expected: Option<usize>) {
            assert_eq!(
                find_in_bitfield(unsafe { avx2_eq_32(needle, haystack.as_ptr()) }, len),
                expected
            );
            assert_eq!(
                find_in_bitfield(unsafe { sse2_eq_32(needle, haystack.as_ptr()) }, len),
                expected
            );
            assert_eq!(find_in_32(needle, haystack, len), expected);
        }

        let mut array = [0; 32];

        test_all(4, &array, 32, None);

        array[4] = 4;
        array[6] = 10;
        array[20] = 5;

        test_all(4, &array, 32, Some(4));
        test_all(5, &array, 32, Some(20));
        test_all(10, &array, 32, Some(6));

        test_all(4, &array, 5, Some(4));
        test_all(5, &array, 5, None);
        test_all(10, &array, 5, None);
    }
}
