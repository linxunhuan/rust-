use std::cmp::{max, Ordering::*};
use std::fmt::{Debug,Display};

// 二叉树的子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 定义二叉树
// key保存数据，left和right 分别保存左、右子节点的地址
#[derive(Debug,Clone,PartialEq)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Debug + Clone + ToString> BinaryTree<T> {
    fn new(key: T) -> Self {
        Self{
            key:key,
            left: None,
            right: None,
        }
    }
    // 将新的子节点作为根节点的左子节点
    fn insert_left_tree(&mut self,key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        }else{
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    // 将新的子节点作为根节点的右子节点
    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        }else{
            let mut  node = BinaryTree::new(key);
            node.left = self.right.take();
            self.left = Some(Box::new(node));
        }
    }
}

impl<T:Clone + Ord + ToString + Debug>BinaryTree<T> {
    // 计算节点数
    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut count: usize) -> usize {
        count += 1;

        // 计算左子节点数
        if let Some(ref  left) = self.left{
            count = left.calc_size(count);
        }

        // 计算右子节点数
        if let Some(ref right) = self.right {
            count = right.calc_size(count);
        }
        count
    }

    // 计算树的深度
    fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left{
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right{
            right_depth += right.depth();
        }

        // 获取左、右子树的深度的最大值
        max(left_depth, right_depth)
    }
}

impl<T:Clone + Ord +ToString + Debug>BinaryTree<T> {
    // 获取左、右子树
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone() 
    }
    
    //获取及设置key
    fn get_key(&self) -> T{
        self.key.clone()
    }
}

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn preorder(& self){
        println!("{:?}", &self.key);
        match &self.left{
            Some(left) => left.preorder(),
            None => (),
        }
        match &self.right{
            Some(right) => right.preorder(),
            None => (),
        }
    }
}

// 前序遍历：外部实现[递归方式]
fn preorder<T:Clone + Ord + ToString + Debug>(bt:Option<Box<BinaryTree<T>>>){
    if !bt.is_none(){
        println!("key:{:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

impl<T:Clone + Ord + ToString + Debug>BinaryTree<T> {
    fn contains(&self){
        match &self.right{
            Some(node) => node.contains(),
            None => (),
        }
        match &self.left{
            Some(node) => node.contains(),
            None => (),
        }
        println!("key: {:?}",&self.key);
    }
}

// 后序遍历：外部实现[递归方式]
fn postorder<T:Clone + Ord + ToString + Debug>(bt: Option<Box<BinaryTree<T>>>){
    if !bt.is_none(){
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
    }
}

impl<T:Clone + Ord + ToString + Debug>BinaryTree<T> {
    fn inorder(& self){
        if self.left.is_some(){
            self.left.as_ref().unwrap().inorder();
        }
        println!("key: {:?}",self.get_key());
        inorder(self.get_right());
    }
}

// 中序遍历：外部实现[递归方式]
fn inorder<T:Clone + Ord + ToString + Debug>(bt:Option<Box<BinaryTree<T>>>){
    if !bt.is_none(){
        inorder(bt.as_ref().unwrap().get_left());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}


impl<T:Clone + Ord + ToString + Debug>BinaryTree<T>{
    // 按照节点位置返回节点组成的字符串表达式：内部实现
    // i:internal,o:outside
    fn iexp(&self) -> String{
        let mut exp = "".to_string();
        let exp_left = match &self.left{
            Some(left) => left.iexp(),
            None => "".to_string(),
        };
        exp += &exp_left;

        exp += &self.get_key().to_string();

        let exp_right = match &self.right{
            Some(right) => right.iexp(),
            None => "".to_string(),
        };

        exp += &exp_right;
        exp += ")";

        exp
    }
}

// 按照节点位置返回节点组成的字符串表达式：外部实现
fn oexp<T>(bt:Link<T>) -> String
    where T:Clone + Ord + ToString + Debug + Display{
        let mut exp = "".to_string();
        if !bt.is_none() {
            exp = "(".to_string() + 
             & oexp(bt.as_ref().unwrap().get_left());
            exp += &(oexp(bt.as_ref().unwrap().get_right())+ ")");
        }
        exp
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_operations() {
        let mut bt = BinaryTree::new(5);

        // 插入左子树
        bt.insert_left_tree(3);
        bt.insert_left_tree(2);
        bt.insert_left_tree(1);

        // 插入右子树
        bt.insert_right_tree(7);
        bt.insert_right_tree(6);
        bt.insert_right_tree(8);

        // 前序遍历
        println!("Preorder traversal:");
        bt.preorder();
        println!();

        // 后序遍历
        println!("Postorder traversal:");
        bt.preorder();
        println!();

        // 中序遍历
        println!("Inorder traversal:");
        bt.inorder();
        println!();

        // 按节点位置返回字符串表达式
        println!("String expression (internal): {}", bt.iexp());
    }

    #[test]
    fn test_binary_tree_levelorder() {
        let mut bt = BinaryTree::new(5);

        bt.insert_left_tree(3);
        bt.insert_left_tree(2);
        bt.insert_left_tree(1);

        bt.insert_right_tree(7);
        bt.insert_right_tree(6);
        bt.insert_right_tree(8);

    }

    #[test]
    fn test_binary_tree_contains() {
        let mut bt = BinaryTree::new(5);

        bt.insert_left_tree(3);
        bt.insert_left_tree(2);
        bt.insert_left_tree(1);

        bt.insert_right_tree(7);
        bt.insert_right_tree(6);
        bt.insert_right_tree(8);

    }
}