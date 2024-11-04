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
#[derive(Debug)]
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
                    let (inserted, deepened) = self.right.insert(key.clone());
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
                    let (inserted, deepened) = self.left.insert(key.clone());
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

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avl_tree(){
        let mut tree = AvlTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);
        println!("{:?}", tree);
    }
}