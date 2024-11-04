// 计算父节点的下标
macro_rules! parent{
    ($parent:ident) => {
        $parent >>1
    };
}

// 计算左子节点的下标
macro_rules! left_child{
    ($parent:ident) => {
        $parent << 1
    };
}

// 计算右子节点的下标
macro_rules! right_child{
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

fn heap_sort(nums:&mut [i32]) {
    if nums.len() < 2 {return;}

    let len = nums.len() - 1;
    let last_parent = parent!(len);
    for i in (1..=last_parent).rev() {
        move_down(nums, i);   // 第一次构建小顶堆，从下标1开始
    }
    for end in (1..nums.len()).rev() {
        nums.swap(1,end);
        move_down(&mut nums[..end-1], 1);// 重建堆
    }
}

// 将大的数据项下移
fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;
    loop{
        let left = left_child!(parent);
        let right = right_child!(parent);
        if left > last { break; } 

        // right =< last ，确保存在右子节点
        let child = if right <= last && nums[right] > nums[left] { right } else { left };

        // 子节点大于父节点，交换数据
        if nums[child] > nums[left]{
            nums.swap(parent, child);
        }

        // 更新父子关系
        parent = child;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut nums = vec![9, 2, 6, 5, 4, 3, 7, 8, 1, 10];
        heap_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}