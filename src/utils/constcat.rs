use std::mem::MaybeUninit;

#[macro_export]
macro_rules! constcat {
    ($($tt:expr),+) => {
        {
            const STR: &str = {
                const LEN: usize = $($tt.len() + )+ 0;
                const LIST_ARR: &[&[u8]] = &[$($tt.as_bytes()),+];

                const ARR: [u8; LEN] = {
                    let arr = $crate::utils::const_concat::<LEN>(LIST_ARR);
                    unsafe { core::mem::transmute(arr) }
                };
                unsafe { core::str::from_utf8_unchecked(&ARR) }
            };
            STR
        }
    };
}

pub const fn const_concat<const LEN: usize>(slices: &[&[u8]]) -> [MaybeUninit<u8>; LEN] {
    let mut arr: [MaybeUninit<u8>; LEN] = [MaybeUninit::uninit(); LEN];
    let mut base = 0;
    let mut i = 0;
    while i < slices.len() {
        let slice = slices[i];
        let mut j = 0;
        while j < slice.len() {
            arr[base + j] = MaybeUninit::new(slice[j]);
            j += 1;
        }
        base += slice.len();
        i += 1;
    }
    if base != LEN {
        panic!("invalid length");
    }
    arr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_const_concat_single() {
        const CONCATENATED: &str = constcat!("SingleString");
        assert_eq!(CONCATENATED, "SingleString");
    }

    #[test]
    fn test_const_concat_multiple() {
        const CONCATENATED: &str = constcat!("This ", "is ", "a ", "test.");
        assert_eq!(CONCATENATED, "This is a test.");
    }

    #[test]
    fn test_const_concat_args() {
        const PART1: &str = "Hello, ";
        const PART2: &str = "World!";
        const CONCATENATED: &str = constcat!(PART1, PART2);
        assert_eq!(CONCATENATED, "Hello, World!");
    }
}
