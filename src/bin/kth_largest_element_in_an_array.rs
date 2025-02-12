pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut x = nums;
    x.sort_unstable();
    *x.get(x.len() - k as usize).unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_findkth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4)
    }
}
