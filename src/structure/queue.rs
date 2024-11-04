#[derive(Debug)]
struct Queue<T>{
    cap:usize,  //容量
    data: Vec<T>,//数据容量
}

impl<T> Queue<T> {
    fn new(size:usize) -> Self {
        Self{
            cap: size,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.len()
    }

    fn is_full(&self)-> bool {
        self.cap == self.len()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    //判断是否有剩余空间，如果有的话，将其数据添加到队列中
    fn enqueue(&mut self, item: T) -> Result<(),String> {
        if self.is_full() {
            return Err("Queue is full!".to_string())
        } else {
            self.data.push(item);
            Ok(())
        }
    }

    //数据出列
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.pop().unwrap())
        }
    }

    /*以下是为队列实现的迭代功能 */
    //队列改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //队列不变，得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter{ stack: Vec::new() };
        for item in self.data.iter(){
            iterator.stack.push(item.clone());
        }
        iterator
    }

    //队列不变，得到可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut{ stack: Vec::new() };
        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }
}

struct IntoIter<T>(Queue<T>);
    impl<T: Clone> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if!self.0.is_empty() {
                self.0.data.pop()
            } else {
                None
            }
        }
    }

struct Iter<'a,T:'a> {
    stack:Vec<&'a T>
}
impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if!self.stack.is_empty() {
            self.stack.pop()
        } else {
            None
        }
    }
}

struct IterMut<'a,T:'a> {
    stack: Vec<&'a mut T>
}
impl<'a,T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if!self.stack.is_empty() {
            self.stack.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new(3);
        assert!(queue.is_empty());
        assert!(!queue.is_full());

        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        assert!(queue.is_full());

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());

        queue.enqueue(4).unwrap();
        queue.enqueue(5).unwrap();
        assert!(!queue.is_full());

        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));

        assert!(queue.is_empty());

        assert_eq!(queue.enqueue(6), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(7), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(8), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(9), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(10), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(11), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(12), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.enqueue(13), Err("Queue is full!".to_string()));
        assert_eq!(queue.dequeue(), None);
    }
}