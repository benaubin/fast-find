#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem::transmute;

use std::is_x86_feature_detected;

use crate::fallback;

pub use crate::fallback::find_in_4;

#[inline]
pub fn find_in_16(needle: u8, haystack: &[u8; 16]) -> usize {
    if is_x86_feature_detected!("avx") {
        let bitfield = unsafe { avx_eq_16(needle, haystack.as_ptr()) };
        find_in_bitfield(bitfield)
    } else {
        fallback::find_in_16(needle, haystack)
    }
}

#[inline]
pub fn find_in_32(needle: u8, haystack: &[u8; 32]) -> usize {
    let bitfield = if is_x86_feature_detected!("avx2") {
        unsafe { avx2_eq_32(needle, haystack.as_ptr()) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { avx_eq_32(needle, haystack.as_ptr()) }
    } else {
        return fallback::find_in_32(needle, haystack);
    };

    find_in_bitfield(bitfield)
}

/// Finds the index of the last 1 in the bitfield, up-to len
///
/// Combines with avx_find_in methods
#[inline]
fn find_in_bitfield(bitfield: i32) -> usize {
    bitfield.trailing_zeros() as usize
}

/// Returns a bitfield, where all elements in haystack that are equal to needle = 1, else 0
///
/// Safety: requires avx, 16 byte haystack
#[inline]
unsafe fn avx_eq_16(needle: u8, haystack: *const u8) -> i32 {
    let haystack = _mm_loadu_si128(haystack as *const __m128i);
    let needle_vec = _mm_set1_epi8(transmute::<u8, i8>(needle));
    let eq = _mm_cmpeq_epi8(needle_vec, haystack);
    _mm_movemask_epi8(eq)
}

/// Returns a bitfield, where all elements in haystack that are equal to needle = 1, else 0
///
/// Safety: requires avx, 32 byte haystack
#[inline]
unsafe fn avx_eq_32(needle: u8, haystack: *const u8) -> i32 {
    avx_eq_16(needle, haystack) | (avx_eq_16(needle, haystack.offset(16)) << 16)
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
    use super::super::bounds_check_idx;
    use super::*;

    #[test]
    fn test_avx_find_16() {
        fn test_all(needle: u8, haystack: &[u8; 16], len: usize, expected: Option<usize>) {
            assert_eq!(
                bounds_check_idx(
                    find_in_bitfield(unsafe { avx_eq_16(needle, haystack.as_ptr()) }),
                    len
                ),
                expected
            );
            assert_eq!(
                bounds_check_idx(find_in_16(needle, haystack), len),
                expected
            );
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
                bounds_check_idx(
                    find_in_bitfield(unsafe { avx2_eq_32(needle, haystack.as_ptr()) }),
                    len
                ),
                expected
            );
            assert_eq!(
                bounds_check_idx(
                    find_in_bitfield(unsafe { avx_eq_32(needle, haystack.as_ptr()) }),
                    len
                ),
                expected
            );
            assert_eq!(
                bounds_check_idx(find_in_32(needle, haystack), len),
                expected
            );
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
