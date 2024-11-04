// 利用二分查找来加快插入排序的排序速度
fn binary_insertion_sort(nums: &mut [i32]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left  = 0;right = i - 1;    // 已排序的左右边界
        
        // 将当前元素存储在'temp'中，也就是待排序的数据
        temp = nums[i];

        // 利用二分查找，找到temp的位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                // 防止出现right = 0 - 1
                if 0 == mid {break;}
                right = mid - 1;
            }else{
                left = mid + 1;
            }
        }

        // 元素后移
        for j in (left..=i - 1).rev() {
            nums.swap(j, j + 1);
        }

        // 将temp插入空位
        if left!= i {nums[left] = temp;}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_insertion_sort() {
        let mut nums = [9, 2, 6, 5, 4, 3, 7, 8, 1];
        binary_insertion_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8,9]);
    }
}