fn merge<T: Ord + Copy>(left: &[T], right: &[T], v: &mut [T]) {
    let mut left_i = 0;
    let mut right_i = 0;
    let mut index = 0;

    while left_i < left.len() && right_i < right.len() {
        if left[left_i] <= right[right_i] {
            v[index] = left[left_i];
            left_i += 1;
        } else {
            v[index] = right[right_i];
            right_i += 1;
        }
        index += 1;
    }

    if left_i < left.len() {
        v[index..].copy_from_slice(&left[left_i..]);
    } else {
        v[index..].copy_from_slice(&right[right_i..]);
    }
}

pub fn mergesort<T: Ord + Copy + Clone>(v: &mut [T]) {
    let mid = v.len() / 2;
    if mid == 0 {
        return;
    }
    mergesort(&mut v[..mid]);
    mergesort(&mut v[mid..]);

    let mut ret = v.to_vec();
    merge(&v[..mid], &v[mid..], &mut ret[..]);
    v.copy_from_slice(&ret);
}