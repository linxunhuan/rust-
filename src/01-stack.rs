#[derive(Debug)]
stuct Stack<T>{
    size: usize,//栈大小
    data: Vec<T>,//栈数据
}

impl<T> Stack<T> {
    // 创建空栈
    fn new() -> Self {
        Self { 
            size:0,
            data:Vec::new()
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }
    
    fn len(&self) -> usize {
        self.size
    }

    //清空栈
    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    //将数据保存在vec的末尾
    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    //在将栈顶减1后，弹出数据
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            Some(self.data.pop().unwrap())
        }
    }

    //返回栈顶数据引用
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        }else {
            Some(&self.data[self.size - 1])
        }
    }

    //返回栈顶数据可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.data[self.size - 1])
        }
    }

    /*以下是为栈实现的迭代功能 */

    //栈改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    struct IntoIter<T>(Stack<T>);
    impl<T:Clone> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if !self.0.is_empty() {
                self.0.size -= 1;
                Some(self.0.data.pop().unwrap())
            }else {
                None
            }
        }
    }
    
    
    //栈不变，得到不可变迭代器
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter{stack:Vector::new()};
        for item in self.data.iter(){
            iterator.stack.push(item.clone());
        } 
        iterator
    }

    struct Iter<'a, T: 'a> {
        stack: Vec<'a, T>,
    }
    impl<'a,T>Iterator for Iter<'a,T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if!self.stack.is_empty() {
                self.stack.pop()
            }else {
    }

    //栈不变，得到可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut{stack:Vec::new()};
        for item in self.data.iter_mut(){
            iterator.stack.push(item.clone());
        }
        iterator
    }

    stuct IterMut<'a, T: 'a> {
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
    use seper::*;

    #[test]
    fn test_basic(){
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.pop(), Some(4));
    }
}