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

    //队列不变，得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        Iter{queue: self.data.clone()}
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

    //队列不变，得到可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{queue: self.data.clone()}
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
}