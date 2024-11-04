fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    //bubble 用于控制是否继续冒泡
    let mut bubble = true;

    let len = nums.len();

    // 二进制右移：将一个数的二进制表示,向右移动指定的位数
    // 例如，8 的二进制表示是 1000，右移 1 位后变为 0100，即 4
    for i in 0..(len >> 1){ 
        
        if bubble {
            bubble = false;

            // 从左往右冒泡
            for j in 0..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            // 从右往左冒泡
            // ..=表明，这是一个左闭右闭的集合
            for j in (i + 1..=(len -i -1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                    bubble = true;
                }
            }
        }else{
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail_sort() {
        let mut nums = vec![5, 3, 8, 2, 1, 9, 4, 7, 6];
        cocktail_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}