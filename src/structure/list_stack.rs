//链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,       // 节点的数据
    next: Link<T>,  // 指向下一个节点的指针
}

// Node自包含引用
type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data:data, next: None }
    }
}

//链表栈
#[derive(Debug,Clone)]
struct LStack<T> {
    size: usize,
    top: Link<T>, // 栈顶元素的引用
}

impl<T:Clone> LStack<T> {
    fn new() -> Self {
        Self { size:0,top: None }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn len(&self) -> usize {
        self.size
    }
    
    fn clear(&mut self){
        self.top = None;
        self.size = 0;
    }

    // 使用take()函数实现入栈
    fn push(&mut self, data: T) {
        let mut node = Node::new(data);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    // 出栈
    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // 返回链表栈中的数据引用
    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    // 返回链表栈中的可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_deref_mut().map(|node| &mut node.data)
    }

    /*为链表栈实现迭代功能 */
    
    //链表栈改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //链表栈不变，只得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        Iter{next: self.top.as_deref()}
    }

    //链表栈不变，得到可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{next: self.top.as_deref_mut()}
    }
}

struct IntoIter<T>(LStack<T>);
impl<T:Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


struct Iter<'a, T: 'a> {next: Option<&'a Node<T>>}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

struct IterMut<'a, T: 'a> {next: Option<&'a mut Node<T>>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lstack() {
        let mut stack = LStack::new();
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.len(), 0);

        stack.push(1);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.len(), 1);

        stack.push(2);
        assert_eq!(stack.len(), 2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }
}