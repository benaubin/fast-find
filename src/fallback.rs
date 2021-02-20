/// Returns the index of the first element in the haystack that matches needle
#[inline]
pub fn find_in_4(needle: u8, haystack: &[u8; 4]) -> usize {
    if haystack[0] == needle {
        0
    } else if haystack[1] == needle {
        1
    } else if haystack[2] == needle {
        2
    } else if haystack[3] == needle {
        3
    } else {
        4
    }
}

pub fn find_in_16(needle: u8, haystack: &[u8; 16]) -> usize {
    // this implementation should help convince llvm to unroll
    let mut pos = 0;
    while haystack[pos] != needle && pos < 16 {
        pos += 1;
    }
    pos
}

pub fn find_in_32(needle: u8, haystack: &[u8; 32]) -> usize {
    // this implementation should help convince llvm to unroll
    let mut pos = 0;
    while haystack[pos] != needle && pos < 32 {
        pos += 1;
    }
    pos
}
