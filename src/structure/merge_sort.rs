fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1{
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[0..mid]); //排序前半部分
        merge_sort(&mut nums[mid..]); // 排序后半部分
        merge(nums, mid); // 合并
    } 
}

fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0;   //标记前半部分数据
    let mut k = mid; // 标记后半部分数据
    let mut temp = Vec::new();

    // 遍历数组，将较小的元素放入临时集合temp中
    for _ in 0..nums.len() {

        // 如果任一部分的数据已经处理完，退出循环
        if k == nums.len() || i == mid{ break;}

        // 将数据放到临时集合temp中
        if nums[i] <= nums[k] {
            temp.push(nums[i]);
            i += 1;
        }else{
            temp.push(nums[k]);
            k += 1;
        }
    }

    // 合并两部数据大概率长度不一样
    // 因此需要将集合中未处理完的数据全部加入
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]); // 将前半部分剩余数据加入 temp
        }
    }else if i == mid && k < nums.len() {
        for j in i..mid{
            temp.push(nums[j]); // 将前半部分剩余数据加入 temp
        }
    }

    // 将 temp 中的数据放回nums，完成排序
    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut nums = vec![9, 2, 6, 5, 4, 3, 7, 8, 1, 10];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}