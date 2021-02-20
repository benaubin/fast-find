#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "arch/x64.rs"]
mod arch;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[path = "fallback.rs"]
mod arch;

mod fallback;

#[inline]
fn bounds_check_idx(idx: usize, len: usize) -> Option<usize> {
    if len > idx {
        Some(idx)
    } else {
        None
    }
}

#[inline]
pub fn find_in_4(needle: u8, haystack: &[u8; 4], len: usize) -> Option<usize> {
    bounds_check_idx(arch::find_in_4(needle, haystack), len)
}

#[inline]
pub fn find_in_16(needle: u8, haystack: &[u8; 16], len: usize) -> Option<usize> {
    bounds_check_idx(arch::find_in_16(needle, haystack), len)
}

#[inline]
pub fn find_in_32(needle: u8, haystack: &[u8; 32], len: usize) -> Option<usize> {
    bounds_check_idx(arch::find_in_32(needle, haystack), len)
}
