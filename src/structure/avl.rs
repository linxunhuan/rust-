use std::cmp::{max, Ordering::*};
use std::fmt::Debug;
use std::mem::replace;
use AvlTree::*;

// AVL树的定义，使用的是枚举
#[derive(Clone, Debug, PartialEq)]
enum AvlTree<T>{
    Tree(Box<AvlNode<T>>),
    Null,
}

// AVL树的节点定义
#[derive(Debug, Clone, PartialEq)]
struct AvlNode<T> {
    key: T,
    left: AvlTree<T>,   // 左子树
    right: AvlTree<T>,  // 右子树
    bfactor: i8,        // 平衡因子     
}

impl<T>AvlTree<T> where T: Ord + Debug + Clone {
    // 新树是空的
    fn new() -> AvlTree<T> {
        Null
    }

    fn insert(&mut self, key: T) -> (bool, bool) {
        let ret = match self{
            // 没有节点，则插入新节点
            Null => {
                let node = AvlNode{
                    key: key,
                    left:Null,
                    right: Null,
                    bfactor: 0,
                };
                *self = Tree(Box::new(node));
                (true, true)
            },

            Tree(ref mut node) => match node.key.cmp(&key){
                // 比较节点数据，判断该从哪边插入
                // inserted 表示是否插入
                // deepened 表示是否加深
                Equal => (false, false), // 相等，无须插入
                Less => {
                    let (inserted, deepened) = node.right.insert(key.clone());
                    if deepened{
                        let ret = match node.bfactor{
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor += 1;

                        ret
                    }else{
                        (inserted, deepened)
                    }
                },
                Greater => {
                    let (inserted, deepened) = node.left.insert(key.clone());
                    if deepened{
                        let ret = match node.bfactor{
                            -1 => (false, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor -= 1;

                        ret
                    }else{
                        (inserted, deepened)
                    }
                }
            }
        };
        self.rebalance();

        ret
    }

    // 调整各节点的平衡因子
    fn rebalance(&mut self) {
        match self{
            Null => {},
            Tree(_) => match self.node().bfactor {
                // 右子树重
                - 2 =>{
                    let lbf = self.node().left.node().bfactor;

                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 {
                            (0, 0)
                        }else{
                            (-1, 1)
                        };

                        // 旋转并更新平衡因子
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    }else if lbf == -1 {
                        let (a, b) = match self.node().left.node().right.node().bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };

                        // 先左旋，再右旋，最后更新平衡因子
                        self.node().left.rotate_left();
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    }else {
                        unreachable!()
                    }
                },
                // 左子树重
                2 =>{
                    let rbf = self.node().right.node().bfactor;
                    if rbf == 1 || rbf == 0{
                        let (a, b) = if rbf == 1 {
                            (0, 0)
                        }else{
                            (1, -1)
                        };

                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    }else {
                        unreachable!()
                    }
                },
                _ => {},
            },
        }
    }

    // 获取节点
    fn node(&mut self) -> &mut AvlNode<T>{
        match self {
            Tree(node) => node,
            Null => panic!("Tree is Empty"),
        }
    }

    // 获取左子树
    fn left_subtree(&mut self) -> &mut Self{
        match self {
            Tree(node) => &mut node.left,
            Null => panic!("Tree is Empty"),
        }
    }

    // 获取右子树
    fn right_subtree(&mut self) -> &mut Self{
        match self {
            Tree(node) => &mut node.right,
            Null => panic!("Tree is Empty"),
        }
    }

    // 进行左旋
    fn rotate_left(&mut self) {
        let mut n = replace(self, Null);
        let mut right = replace(n.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *n.right_subtree() = right_left;
        *right.left_subtree() = n;
        *self = right;
    }

    // 进行右旋
    fn rotate_right(&mut self) {
        let mut n = replace(self, Null);
        let mut left = replace(n.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *n.left_subtree() = left_right;
        *left.right_subtree() = n;
        *self = left;
    }
}


impl<T> AvlTree<T> where T:Ord + Debug + Clone{
    // 计算数的节点数：左/右子节点数 + 跟节点数，递归计算
    fn size(&self) -> usize{
        match self {
            Tree(n) => 1 + n.left.size() + n.right.size(),
            Null => 0,
        }
    }

    // 计算叶节点数
    fn leaf_size(&self) -> usize{
        match self {
            Null => 0,
            Tree(node) => {
                if node.left == Null && node.right == Null{
                    return 1;
                }
                let left_leaf = match node.left {
                    _ => node.left.leaf_size(),
                    Null => 0,
                };
                let right_leaf = match node.right {
                    _ => node.right.leaf_size(),
                    Null => 0,
                };
                left_leaf + right_leaf
            },
        }
    }

    // 计算非叶节点数
    fn none_leaf_size(&self) -> usize{
        self.size() - self.leaf_size()
    }

    // 树的深度等于左、右子树的深度最大值 + 1， 递归运算
    fn depth(&self) -> usize{
        match self {
            Null => 0,
            Tree(node) => max(node.left.depth(), node.right.depth()) + 1,
        }
    }

    fn is_empty(&self) -> bool{
        match self {
            Null => true,
            _ => false,
        }
    }

    // 找到树的最小节点值
    fn min(&self) -> Option<&T>{
        match self {
            Null => None,
            Tree(node) => {
                match node.left {
                    _ => node.left.min(),
                    Null => Some(&node.key),
                }
            },
        }
    }

    // 获取树的最大节点值
    fn max(&self) -> Option<&T>{
        match self {
            Null => None,
            Tree(node) => {
                match node.right {
                    _ => node.right.max(),
                    Null => Some(&node.key),
                }
            }
        }
    }

    // 查找节点
    fn contains(&self, key:&T) -> bool{
        match self {
            Null => false,
            Tree(node) => match node.key.cmp(key){
                Equal => true,
                Less => {
                    match &node.right {
                        Null => false,
                        _ => node.right.contains(key),
                    }
                },
                Greater => {
                    match &node.left {
                        Null => false,
                        _ => node.left.contains(key),
                    }

                },
            },
        }
    }

    // 前序遍历
    fn preorder(&self){
        match self {
           Null => (),
           Tree(node) => {
            print!("key:{:?}", node.key);
            node.left.preorder();
            node.right.preorder();
           } 
        }
    }

    // 中序遍历
    fn inorder(&self){
        match self {
           Null => (),
           Tree(node) => {
            node.left.inorder();
            print!("key:{:?}", node.key);
            node.right.inorder();
           } 
        }
    }

    // 后序遍历
    fn postorder(&self){
        match self {
           Null => (),
           Tree(node) => {
            node.left.postorder();
            node.right.postorder();
            print!("key:{:?}", node.key);
           } 
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_avl_tree(){
        let mut tree = AvlTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(8);
        tree.insert(2);
        tree.insert(4);
        tree.insert(7);
        tree.insert(10);
        tree.insert(6);
        tree.insert(9);
        tree.insert(1);

        assert_eq!(tree.size(), 10);
        assert_eq!(tree.leaf_size(), 3);
        assert_eq!(tree.none_leaf_size(), 7);
        assert_eq!(tree.depth(), 4);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&15), false);

        tree.preorder();
        println!("\n");
    }

    #[test]
    fn test_avl_tree_min_max(){
        let mut tree = AvlTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(8);
        tree.insert(2);
        tree.insert(4);
        tree.insert(7);
        tree.insert(10);
        tree.insert(6);
        tree.insert(9);
        tree.insert(1);

        assert_eq!(tree.min().unwrap(), &1);
        assert_eq!(tree.max().unwrap(), &10);
    }
}