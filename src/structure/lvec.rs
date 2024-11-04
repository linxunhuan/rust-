use std::fmt::Debug;

// 节点
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node { elem:elem, next: None }
    }
}

// 链表vec
#[derive(Debug)]
struct LVec<T> {
    head: Link<T>,
    size: usize,
}

impl<T:Copy + Debug> LVec<T> {
    fn new() -> Self {
        LVec { head: None, size: 0 }
    }
    
    fn is_empty(&self) -> bool{
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size   
    }

    fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    fn push(&mut self, elem: T) {
        let node = Node::new(elem);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        }else{
            let mut curr = self.head.as_mut().unwrap();

            // 找到链表中的最后一个节点
            for _ in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 在最后一个节点的后面插入新的数据
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    // 在栈尾添加新的LVec
    fn append(&mut self, other: &mut LVec<T>) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear();
    }

    fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size {
            index = self.size;
        }

        let mut node = Node::new(elem);

        if self.is_empty() {
            self.head = Some(Box::new(node));
        }else if index == 0 {
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        }else{
            let mut curr = self.head.as_mut().unwrap();

            // 找到要插入的位置的前一个节点
            for _ in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 插入新的数据
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T>{
        if self.is_empty() {
            None
        } else {
            self.remove(self.size - 1)
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        // 分两种情况删除其他节点，头节点的删除最好处理
        let mut node;
        if index == 0 {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        }else{
            let mut curr = self.head.as_mut().unwrap();

            // 找到要删除的位置的前一个节点
            for _ in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 删除并返回被删除的数据
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;
        Some(node.elem)
    }

    /*为栈实现的迭代功能 */
    // 栈改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }


    // 栈不变，只得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        Iter{next: self.head.as_deref()}
    }



    // 栈不变，得到可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{next: self.head.as_deref_mut()}
    }

    // 输出LVec中的数据项
    fn print_lvec(&self) {
        if self.is_empty() {
            println!("LVec is empty");
            return;
        }else{
            for item in self.iter() {
                println!("{:?}", item);
            }
        }
    }
}

struct IntoIter<T:Copy + Clone + Debug>(LVec<T>);
impl<T: Copy + Clone + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a,T:'a>{next: Option<&'a Node<T>>}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

struct IterMut<'a, T:'a> {next: Option<&'a mut Node<T>>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lvec_push_and_pop() {
        let mut lvec = LVec::new();
        lvec.push(1);
        lvec.push(2);
        lvec.push(3);

        assert_eq!(lvec.pop(), Some(3));
        assert_eq!(lvec.pop(), Some(2));
        assert_eq!(lvec.pop(), Some(1));
        assert_eq!(lvec.pop(), None);
        lvec.push(4);
        assert_eq!(lvec.pop(), Some(4));
        assert_eq!(lvec.pop(), None);

        lvec.clear();
        assert_eq!(lvec.is_empty(), true);
        assert_eq!(lvec.len(), 0);    
    }
}