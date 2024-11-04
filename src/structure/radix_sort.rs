fn radix_sort(nums:&mut [usize]){
    if nums.len() <= 1 { return;}

    // 找到最大值。得到位数
    let max_num = match nums.iter().max() {
        Some(&x) => x,
        None => return,
    };

    // 寻找最接近且大于等于nums序列长度的2的幂值作为桶大小，例如：
    // 最接近且大于等于10 的2的幂值是2^4=16
    // 最接近且大于等于17 的2的幂值是2^5=32
    let radix = nums.len().next_power_of_two();

    // digit代表桶内元素的个数
    // 个、十、百、千分别对应1、2、3、4
    // 排序从个位开始，所以digit为1
    let mut digit = 1;
    
    while digit < max_num{
        // 计算数据在桶中的哪个位置
        let index_of = |num| num % digit;

        // 计数器
        let mut counter = vec![0; radix];
        
        // 计数，统计每个桶中的元素个数
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        // 累计计数，使其表示位置索引
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        // 排序
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }
        // 跨越桶
        digit *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut nums = [170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut nums);
        assert_eq!(nums, [2, 24, 45, 66, 75, 90, 170, 802]);
        println!("{:?}", nums);
    }
}