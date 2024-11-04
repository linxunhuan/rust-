// 计算父节点的下标
macro_rules! parent{
    ($child:ident) => {
        $child >> 1
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

// 定义二叉堆
#[derive(Debug,Clone)]
struct BinaryHeap{
    size: usize, // 数据量
    data: Vec<i32>, // 数据容量
}

impl BinaryHeap{
    fn new() -> Self{
        BinaryHeap{
            size: 0,    // 将vec的首位置0，但不计入总数
            data: vec![0],
        }
    }

    fn size(&self) -> usize{
        self.size
    }

    fn is_empty(&self) -> bool{
        self.size == 0
    }

    // 获取堆中最小数据
    fn min(&self) -> Option<&i32>{
        if self.is_empty(){
            None
        }else{
            Some(&self.data[1])
        }
    }
}

impl BinaryHeap{
    // 在对的末尾加入一个数据，调整堆
    fn push(&mut self, value: i32){
        self.data.push(value);
        self.size += 1;
        self.move_up(self.size);
    }

    // 将小的数据往上移动，类似于冒泡
    fn move_up(&mut self, mut child: usize){
        loop{
            // 计算当前节点的父节点的位置
            let p = parent!(child);
            if p <= 0 {break;}

            // 当前节点数据小于父节点数据，交换
            if self.data[p] > self.data[child]{
                self.data.swap(child, p);
            }
            // 父节点成为当前节点
            child = p;
        }
    }

    fn pop(&mut self) -> Option<i32>{   // 获取堆顶数据
        if 0 == self.size{  
            None
        }else if 1 == self.size{
            self.size -= 1; // 堆中只有一个数据，比较好处理
            self.data.pop()
        }else{
            // 堆中有多个数据，先交换并弹出数据，再调整堆
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            val
        }
    }

    // 大的数据下沉
    fn move_down(&mut self, mut child: usize){
        loop{

            // 当前节点的左子节点的位置
            let lc = left_child!(child);
            if lc > self.size{ break;}

            // 当前节点的最小子节点的位置
            let mc = self.min_child(child);
            if self.data[child] > self.data[mc]{
                self.data.swap(child, mc);
            }

            // 最小子节点成为当前节点
            child = mc;
        }
    }

    // 计算最小子节点的位置
    fn min_child(&self, child: usize) -> usize{
        let (lc, rc) = (left_child!(child), right_child!(child));

        if rc > self.size{
            // 右子节点的位置 > size,左子节点是最小子节点
            lc
        }else if self.data[lc] < self.data[rc]{
            // 存在左、右子节点，需要判断左、右子节点中的哪个子节点更小
            lc
        }else{
            // 右子节点是最小子节点
            rc
        }
    }

    fn build_new(&mut self, arr:&[i32]){
        // 删除原始数据
        for _i in 0..self.size{
            let _rm = self.data.pop();
        }

        // 添加新数据
        for &val in arr{
            self.push(val);
        }
        self.size = arr.len();

        // 调整堆，使其成为小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0{
            self.move_down(p);
            p -= 1;
        }
    }

    // 将切片数据逐个加入堆
    fn build_add(&mut self, arr: &[i32]){
        for &val in arr{
            self.push(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_heap(){
        let mut bh = BinaryHeap::new();
        bh.build_add(&[3, 2, 1, 5, 4]);
        assert_eq!(bh.min().unwrap(), &1);
        assert_eq!(bh.pop(), Some(1));
        assert_eq!(bh.pop(), Some(2));
        assert_eq!(bh.pop(), Some(3));
        assert_eq!(bh.pop(), Some(4));
        assert_eq!(bh.pop(), Some(5));
        assert_eq!(bh.pop(), None);
    }
}