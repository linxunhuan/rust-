fn comb_sort(nums:&mut [i32]){
    if nums.len() <= 1 { return;}
    let mut i;
    let mut gap:usize = nums.len();

    // 大致排序，数据基本有序
    while gap > 0{
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len(){
            if nums[i - gap] > nums[i]{
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    // 细致地调节部分无序数据
    // exchange变量用于控制是否继续交换数据
    let mut exchange = true;
    while exchange{
        exchange = false;
        i = 0;

        // 遍历数组，进行冒泡排序
        while i < nums.len() - 1{
            if nums[i] > nums[i + 1]{
                nums.swap(i, i + 1);
                exchange = true;
            }
            i += 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_sort() {
        let mut nums = vec![5, 3, 8, 2, 1, 9, 4, 7, 6];
        comb_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}