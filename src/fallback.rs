#[inline]
fn find_in(needle: u8, haystack: &[u8], len: usize) -> Option<usize> {
    let idx = haystack.iter().position(|el| *el == needle)?;
    if len > idx {
        Some(idx)
    } else {
        None
    }
}

/// Returns the index of the first element in the haystack that matches needle
#[inline]
pub fn find_in_4(needle: u8, haystack: &[u8; 4], len: usize) -> Option<usize> {
    find_in(needle, haystack, len)
}

pub fn find_in_16(needle: u8, haystack: &[u8; 16], len: usize) -> Option<usize> {
    find_in(needle, haystack, len)
}

pub fn find_in_32(needle: u8, haystack: &[u8; 32], len: usize) -> Option<usize> {
    find_in(needle, haystack, len)
}



    }
}
