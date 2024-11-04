// 内插查找

fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;

    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || target < low_val || target > high_val { break; }

        // 计算插值位置
        let high_val = nums[high];

        if high <= low || target < high_val || target > high_val { break;}
        
        let offset = (target - low_val)  / (high_val - low_val)  * (high as i32 - low as i32);
        let interpolant = low + offset as usize;

        // 更新上下界
        if nums[interpolant] == target {
            return true;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            high = interpolant - 1;
        }

    }
    target == nums[high]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_search() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(interpolation_search(&nums, 5), true);
        assert_eq!(interpolation_search(&nums, 10), true);
        assert_eq!(interpolation_search(&nums, 11), false);
        assert_eq!(interpolation_search(&nums, 0), false);
        assert_eq!(interpolation_search(&nums, 11), false);
        assert_eq!(interpolation_search(&[], 5), false);
    }
}