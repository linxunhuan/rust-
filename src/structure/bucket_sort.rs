use std::fmt::Debug;

struct Bucket <H,T>{
    hasher:H,       // hasher 是一个哈希函数，在计算时传入
    values:Vec<T>,  // valus 是数据容器，用于保存数据
}

impl<H, T> Bucket<H, T>{
    fn new(hasher:H, value:T) -> Bucket<H, T>{
        Bucket{
            hasher:hasher,
            values: vec![value],
        }
    }
}

// 桶排序
fn bucket_sort<H, T, F>(nums: &mut [T],hasher: F)
    where H: Ord,
          T: Debug + Ord + Clone, 
          F: Fn(&T) -> H,           // F 是一个函数类型，接受&T类型并返回 H 类型
{
    let mut buckets:Vec<Bucket<H, T>> = Vec::new(); // 备桶
    for val in nums.iter() {
        let hasher = hasher(&val);

        // 对同种的数据进行二分查找并排序
        match buckets.binary_search_by(|bct|
        bct.hasher.cmp(&hasher)) {              // 比较桶中的哈希值的桶
            Ok(idx) => buckets[idx].values.push(val.clone()),  
            Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }
    // 拆桶，将所有所有排序数据合并到一个vec中
    let ret = buckets.into_iter().flat_map(|mut bucket|{
        bucket.values.sort();
        bucket.values  
    }).collect::<Vec<T>>();

    // 将排序结果复制回原数组
    nums.clone_from_slice(&ret);
}

#[cfg(tests)]
mod tests{
    use super::*;
    #[test]
    fn test_bucket_sort(){
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        bucket_sort(&mut nums, |x| x % 3);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}