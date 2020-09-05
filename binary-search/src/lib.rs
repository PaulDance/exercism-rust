pub fn find<T: PartialOrd + PartialEq>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.len() == 0 {
        None
    } else {
        let mut l = 0;
        let mut r = array.len() - 1;

        while l <= r {
            let m = (l + r) / 2;

            if array[m] == key {
                return Some(m);
            } else if array[m] > key {
                if m == 0 {
                    return None;
                } else {
                    r = m - 1;
                }
            } else {
                l = m + 1;
            }
        }

        None
    }
}
