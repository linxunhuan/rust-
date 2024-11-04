/**
 * Rust实现二叉查找树
 * 二叉查找树的左子节点的键必须小于父节点的键
 * 右子节点的键必须大于父节点的键
 */
use std::cmp::{max, Ordering::*};
use std::fmt::Debug;

// 子节点链接
type Link<Y,U> = Option<Box<BST<Y,U>>>;

// 定义二叉搜索树
#[derive(Debug, Clone, PartialEq)]
pub struct BST<Y, U> {
    key: Option<Y>,
    value: Option<U>,
    left: Link<Y, U>,
    right: Link<Y, U>,
}

impl<T, U>BST<T, U> 
where T: Ord + Clone + Debug, U: Clone {
    fn new() -> Self{
        Self{
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool{
        self.key.is_none()
    }

    // 递归计算节点数
    fn calc_size(&self, mut size: usize) -> usize{
        if self.key.is_none(){
            return size;
        }

        // 将当前节点数加入总结点数
        size += 1;

        // 计算左、右子节点数
        if !self.left.is_none(){
            size = self.left.as_ref().unwrap().calc_size(size);
        }

        if!self.right.is_none(){
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    fn size(&self) -> usize{
        self.calc_size(0)
    }

    // 计算叶节点数
    fn leaf_size(&self) -> usize {
        // 都为空，当前节点就是叶节点，返回1
        if self.left.is_none() && self.right.is_none(){
            return 1;
        }

        // 计算左右子树的叶节点数
        let left_leaf = match &self.left{
            Some(node) => node.leaf_size(),
            None => 0,
        };

        let right_leaf = match &self.right{
            Some(node) => node.leaf_size(),
            None => 0,
        };

        // 左右子树的叶节点数之和，就是当前节点的叶节点数
        left_leaf + right_leaf
    }

    // 计算非叶节点数
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    // 计算树的深度
    fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        max(left_depth, right_depth)
    }

    // 插入节点
    fn insert(&mut self,key:T,value:U){
        // 没有数据时，直接插入
        if self.key.is_none(){
            self.key = Some(key.clone());
            self.value = Some(value.clone());
        }else{
            match&self.key{
                Some(k) =>{
                    // 存在key，更新val
                    if key == *k{
                        self.value = Some(value.clone());
                        return;
                    }

                    // 未找到相同的key，需要插入新节点
                    // 先找到需要插入的子树
                    let child = if key <= *k {
                        &mut self.left
                    }else{
                        &mut self.right
                    };

                    // 根据节点递归下去，直到插入为止
                    match child{
                        Some(ref mut node) =>{
                            node.insert(key.clone(), value.clone());
                        }
                        None =>{
                            let mut node = BST::new();
                            node.insert(key.clone(), value.clone());
                            *child = Some(Box::new(node));
                        },
                    }
                },
                None => (),
            }
        }
    }

    // 查询节点
    fn contains(&self, key: &T) ->bool{
        match &self.key{
            None => false,
            Some(k) => {
                // 判断是否继续递归查询
                match k.cmp(key) {
                    // 判断是否继续递归查询
                    Equal => true, //找到数据
                    Greater =>{
                        match &self.left{   // 在左子树中搜索
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    },
                    Less => {
                        match &self.right{ // 在右子树中搜索
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    },
                }
            },
        }
    }

    // 求最小/最大节点值
    fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在最左侧
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(key) => (Some(key), self.value.as_ref()),
                None => (None, None),
            },
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在最右侧
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(key) => (Some(key), self.value.as_ref()),
                None => (None, None),
            },
        }
    }

    // 获取左、右子节点
    fn get_left(&self) -> Link<T, U>{
        self.left.clone()
    }

    fn get_right(&self) -> Link<T, U>{
        self.right.clone()
    }

    // 获取值引用，与查找流程相似
    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            Some(k) => {
                match k.cmp(key){
                    Equal => Some(self.value.as_ref()?),
                    Greater => match &self.left {
                        Some(node) => node.get(key),
                        None => None,
                    },
                    Less => match &self.right {
                        Some(node) => node.get(key),
                        None => None,
                    },
                }
            },
            None => None,
        }
    }
}

// 前序遍历、中序遍历、后序遍历和层序遍历：内部实现
impl<T, U>BST<T,U>
    where T: Ord + Copy + Debug, U: Copy + Debug{
        fn preorder(&self){
            println!("key:{:?}, val:{:?}",self.key,self.value);
            match &self.left{
                Some(node) => node.preorder(),
                None => (),
            }

            match &self.right{
                Some(node) => node.preorder(),
                None => (),
            }
        }

        fn inorder(&self){
            match &self.left{
                Some(node) => node.inorder(),
                None => (),
            }
            println!("key:{:?}, val:{:?}",self.key,self.value);
            match &self.right{
                Some(node) => node.inorder(),
                None => (),
            }
        }

        fn postorder(&self){
            match &self.left{
                Some(node) => node.postorder(),
                None => (),
            }

            match &self.right{
                Some(node) => node.postorder(),
                None => (),
            }
            println!("key:{:?}, val:{:?}",self.key, self.value);
        }

        fn levelorder(&self){
            let size = self.size();
            let mut queue = Queue::new(size);

            let _r = queue.enqueue(Box::new(self.clone()));
            while !queue.is_empty(){
                let front = queue.dequeue().unwrap();
                println!("key:{:?}, val:{:?}", front.key, front.val);

                match front.key.get_left() {
                    Some(left) => { let _r = queue.enqueue(left);},
                    None => (),
                }

                match front.key.get_right() {
                    Some(right) => { let _r = queue.enqueue(right);},
                    None => (),
                }
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BST::new();
        bst.insert(5, 5);
        bst.insert(3, 3);
        bst.insert(8, 8);
        bst.insert(2, 2);
        bst.insert(4, 4);
        bst.insert(7, 7);
        bst.insert(9, 9);

        assert_eq!(bst.size(), 8);
        assert_eq!(bst.leaf_size(), 3);
        assert_eq!(bst.none_leaf_size(), 5);
        assert_eq!(bst.depth(), 3);

        assert_eq!(bst.contains(&3), true);
        assert_eq!(bst.contains(&10), false);

        assert_eq!(bst.min().0, Some(&1));
        assert_eq!(bst.max().0, Some(&9));
        assert_eq!(bst.get(&3), Some(&3));
        assert_eq!(bst.get(&10), None);
        assert_eq!(bst.get_left(), None);
        assert_eq!(bst.get_right(), None);
        assert_eq!(bst.get_left().as_ref().and_then(|node| node.get(&2)), Some(&2));
        assert_eq!(bst.get_right().as_ref().and_then(|node| node.get(&7)), Some(&7));
        assert_eq!(bst.get_right().as_ref().and_then(|node| node.get(&10)), None);
        assert_eq!(bst.get_right().as_ref().and_then(|node| node.get_left()), None);
        assert_eq!(bst.get_right().as_ref().and_then(|node| node.get_right()), None);

        bst.preorder();
        bst.inorder();
        bst.postorder();
        bst.levelorder();
    }
}
