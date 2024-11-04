fn quick_sort(nums: &mut [i32], low:usize, high:usize) {
    if low < high {
        let split = partition(nums, low, high);

        // 防止越界（split <=1）和语法错误
        if split > 1 {
            quick_sort(nums, low, split - 1);
        }
        quick_sort(nums, split + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low; let mut rm = high;

    loop {
        // 左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm < rm && nums[lm] >= nums[low] {
            rm -= 1;
        }
        // 交换左右标记的值
        if lm < rm {
            nums.swap(lm, rm);
        } else {
            break;
        }
    }
    nums.swap(low, rm);
    return rm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut nums = [9, 2, 6, 5, 4, 3, 7, 8, 1];
        let len = nums.len();
        quick_sort(&mut nums, 0, len- 1);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8,9]);
    }
}