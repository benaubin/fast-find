/// Returns the index of the first element in the haystack that matches needle, or an out of bounds index if not found
#[inline]
fn find_in(needle: u8, haystack: &[u8]) -> usize {
    let mut i = 0;

    while i < haystack.len() {
        if haystack[i] == needle { break }
        i += 1;
    };

    i
}

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
#[inline]
pub fn find_in_4(needle: u8, haystack: &[u8; 4]) -> usize {
    find_in(needle, haystack)
}

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
pub fn find_in_16(needle: u8, haystack: &[u8; 16]) -> usize {
    find_in(needle, haystack)
}

/// Returns the index of the first element in the haystack that matches needle, up-to len, or None
pub fn find_in_32(needle: u8, haystack: &[u8; 32]) -> usize {
    find_in(needle, haystack)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_4() {
        let mut array = [0; 4];

        assert_eq!(find_in_4(4, &array), 4);

        array[1] = 4;
        array[3] = 10;

        assert_eq!(find_in_4(4, &array), 1);
        assert_eq!(find_in_4(5, &array), 4);
        assert_eq!(find_in_4(10, &array), 3);
    }

    #[test]
    fn test_find_16() {
        let mut array = [0; 16];

        assert_eq!(find_in_16(4, &array), 16);

        array[4] = 4;
        array[6] = 10;

        assert_eq!(find_in_16(4, &array), 4);
        assert_eq!(find_in_16(5, &array), 16);
        assert_eq!(find_in_16(10, &array), 6);
    }

    #[test]
    fn test_find_32() {
        let mut array = [0; 32];

        assert_eq!(find_in_32(4, &array), 32);

        array[4] = 4;
        array[6] = 10;
        array[20] = 5;

        assert_eq!(find_in_32(4, &array), 4);
        assert_eq!(find_in_32(5, &array), 20);
        assert_eq!(find_in_32(10, &array), 6);
    }
}
