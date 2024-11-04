fn bubble_sort(nums: &mut [i32]) {
    // compare 用于控制是否继续比较
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true;
            }
        }
        len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums = vec![5, 3, 8, 2, 1, 9, 4, 7, 6];
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}