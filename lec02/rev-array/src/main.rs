// initial implementation
/* fn rev_arr<T: Clone + Copy, const N: usize>(mut arr: [T; N]) -> [T; N] {
    let clone = arr.clone();
    let mut index = arr.len();
    for i in 0..arr.len() {
        arr[i] = clone[index - 1];
        index -= 1;
    }
    arr
} */

//better implementation
fn rev_arr<T, const N: usize>(mut arr: [T; N]) -> [T; N] {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
    arr
}

fn rev_arr_slice<T>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

fn main() {
    let mut array: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", rev_arr(array));

    let mut other_array = [5,6,7,8,9];
    rev_arr_slice(&mut other_array);
    println!("{:?}", other_array);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_arr_even_len() {
        let arr = [8, 7, 6, 5, 4, 3];
        let expected = [3, 4, 5, 6, 7, 8];
        assert_eq!(rev_arr(arr), expected);
    }

    #[test]
    fn test_rev_arr_odd_len() {
        let arr = [8, 7, 6, 5, 4];
        let expected = [4, 5, 6, 7, 8];
        assert_eq!(rev_arr(arr), expected);
    }

    #[test]
    fn test_rev_arr_no_len() {
        let arr: [i32; 0] = [];
        let expected: [i32; 0] = [];
        assert_eq!(rev_arr(arr), expected);
    }

    #[test]
    fn test_rev_arr_diff_type() {
        let arr = ["cat", "dog", "frog"];
        let expected = ["frog", "dog", "cat"];
        assert_eq!(rev_arr(arr), expected);
    }

    #[test]
    fn test_rev_arr_identity_law_odd() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(rev_arr(rev_arr(arr)), arr);
    }

    #[test]
    fn test_rev_arr_identity_law_even() {
        let arr = [2, 3, 4, 5];
        assert_eq!(rev_arr(rev_arr(arr)), arr);
    }
} 