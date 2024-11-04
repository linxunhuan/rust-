type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    size: usize,    //链表中的节点数
    head: Link<T>,  //头节点
}

//链表节点
struct Node<T> {
    elem: T,       // 节点的数据
    next: Link<T>,  // 指向下一个节点的指针
}

impl<T> List<T> {
    fn new() -> Self {
        List { size: 0, head: None }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
    
    // 新节点被插入到头部
    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node { elem:elem, next: self.head.take() });
        self.head = Some(new_node);
        self.size += 1;
    }

    //take()会取出数据，留下空位
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;
            old_head.elem
        })
    }

    //peek()不改变值，只能是引用
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    //peek_mut()可改变值，是可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /*以下是为链表实现的迭代器的功能 */
    //链表改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //链表不变，得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        Iter{next: self.head.as_deref()}
    }

    //链表不变，得到可变迭代
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{next: self.head.as_deref_mut()}
    }
}

struct IntoIter<T>(List<T>);
impl<T: Clone> Iterator for IntoIter<T> {
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
            &node.elem
        })
    }
}

struct IterMut<'a, T: 'a> {next: Option<&'a mut Node<T>>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
    // take() 函数在这里的作用是将 Option 类型的值从 self.next 中取出，同时将 self.next 设置为 None
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

//为链表实现自定义Drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.len(), 0);

        list.push(1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.peek(), Some(&1));

        list.push(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek(), Some(&2));

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        list.clear();
        assert_eq!(list.len(), 0);
    }
}