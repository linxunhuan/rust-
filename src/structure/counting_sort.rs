fn counting_sort(nums:&mut [usize]){
    if nums.len() <= 1{
        return;
    }

    // 桶的数量等于nums中的最大值加1，以保证所有数据都有桶来存放
    let max_bkt_num = 1 + nums.iter().max().unwrap();

    // 将数据标记到桶中
    let mut counter = vec![0;max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1;
    }

    // 将数据写回 nums 切片
    let mut j = 0;              // 用于记录当前写入nums的位置
    for i in 0..max_bkt_num {   
        while counter[i] > 0 {         // 当桶中还有元素时
            nums[j] = i;        // 将当前桶的索引i 写入nums
            counter[i] -= 1;     
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_counting_sort(){
        let mut nums = [4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut nums);
        assert_eq!(nums, [1, 2, 2, 3, 3, 4, 8]);
        println!("{:?}", nums);
    }
}