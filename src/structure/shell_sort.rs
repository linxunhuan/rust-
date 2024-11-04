fn shell_sort(nums: &mut [i32]) {
    // 插入排序（内部），数据相隔的距离为gap
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;
        // 从gap位置开始遍历，直到数组末尾
        while i < nums.len() {
            let mut pos = i;    // 当前元素的位置
            let curr = nums[pos]; // 将当前元素存储在‘curr’中

            // 将当前元素与之前的元素(相隔gap)进行比较并插入到正确的位置
            while pos >= gap && nums[pos - gap] > curr {
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }
            nums[pos] = curr;
             i += gap;
        }
    }

    // 初始的gap值为数组长度的一半
    let mut gap = nums.len() / 2;

    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut nums = [4, 5, 6, 3, 2, 8];
        shell_sort(&mut nums);
        assert_eq!(nums, [2, 3, 4, 5, 6, 8]);
    }
}