#[derive(Debug, Clone, PartialEq)]
struct HashMap <T>{
    cap:usize,          // 控制容器
    slot:Vec<usize>,    // 保存键
    data:Vec<T>,        // 保存值
}

impl<T:Clone + PartialEq + Default> HashMap<T>{
    fn new(cap:usize) -> Self {
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for _ in 0..cap {
            slot.push(0);
            data.push(Default::default());
        }

        HashMap {cap, slot, data,}
    }

    fn len(&self) -> usize{
        let mut len = 0;
        for &index in self.slot.iter() {
            if index!= 0 {
                len += 1;
            }
        }
        len
    }

    fn is_empty(&self) -> bool {
        let mut empty = true;
        for &index in self.slot.iter() {
            if index!= 0 {
                empty = false;
                break;
            }
        }
        empty
    }

    fn clear(&mut self) {
        let mut slot = Vec::with_capacity(self.cap);
        let mut data = Vec::with_capacity(self.cap);
        for _ in 0..self.cap {
            slot.push(0);
            data.push(Default::default());
        }
        self.slot = slot;
        self.data = data;
    }

    fn hash(&self,key:usize) -> usize {
        key % self.cap
    }

    fn rehash(&self,pos:usize) -> usize {
        (pos + 1) % self.cap
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key {panic!("Error:key must > 0");}

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            self.slot[pos] = key;
            self.data[pos] = value;
            return;
        }else{
            let mut next = self.rehash(pos);
            while self.slot[next]!= 0 
                && self.slot[next]!= key {
                    next = self.rehash(next);

                    if next == pos {
                        panic!("Error:slot is full");
                    }
                }
            
            if self.slot[next] == 0 {
                self.slot[next] = key;
                self.data[next] = value;
            }else{
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {panic!("Error:key must > 0");}

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            None
        }else if self.slot[pos] == key {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        }else{

            // 如果pos的槽位不是要删除的键，则需要处理冲突
            let mut data : Option<T> = None;
            let mut stop = false;
            let mut curr = pos;

            while self.slot[curr]!= 0 && !stop {
                
                // 找到了值删除数据
                if key == self.slot[curr] {
                    stop = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                }else{
                    // 哈希回到最初的位置，说明找了一圈没找到
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    fn get_pos(&self, key:usize) -> usize{
        if 0 == key {panic!("Error:key must > 0");}

        // 计算数据的位置
        let pos = self.hash(key);
        let mut stop = false;
        let mut curr = pos;

        // 循环查找数据
        while self.slot[curr]!= 0 &&!stop {
            if key == self.slot[curr] {
                stop = true;
            } else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        curr
    }

    // 获取val的引用

    fn get(&self, key: usize) -> Option<&T> {
        let curr = self.get_pos(key);
        self.data.get(curr)
    }

    // 获取val的可变引用
    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
         let curr = self.get_pos(key);
         self.data.get_mut(curr)
    }

    fn contains(&self, key: usize) -> bool {
        if key == 0 {panic!("Error:key must > 0");}

        self.slot.contains(&key)
    }

    //  迭代器
    fn iter(&self) -> Iter<T>{
        let mut iterator = Iter{stack:Vec::new()};
        for item in self.data.iter() {
            iterator.stack.push(item.clone());
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T>{
        let mut iterator = IterMut{stack:Vec::new()};
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// 实现迭代器功能
struct Iter<T> { stack: Vec<T>,}

impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T>,}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map() {
        let mut map = HashMap::new(10);
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");
        map.insert(4, "d");
        map.insert(5, "e");

        assert_eq!(map.len(), 5);
        assert_eq!(map.is_empty(), false);

        assert_eq!(map.get(1), Some(&"a"));
        assert_eq!(map.get(2), Some(&"b"));
        assert_eq!(map.get(3), Some(&"c"));
        assert_eq!(map.get(4), Some(&"d"));
        assert_eq!(map.get(5), Some(&"e"));

        assert_eq!(map.get(6), None);

        map.remove(2);

        assert_eq!(map.get(2), None);
        assert_eq!(map.len(), 4);
        assert_eq!(map.is_empty(), false);
        assert_eq!(map.contains(2), false);
        assert_eq!(map.contains(1), true);
        assert_eq!(map.contains(3), true);
        assert_eq!(map.contains(4), true);
        assert_eq!(map.contains(5), true);
        assert_eq!(map.contains(6), false);
        assert_eq!(map.iter().collect::<Vec<&str>>(), vec!["a", "c", "d", "e"]);
        map.clear();
        assert_eq!(map.len(), 0);
    }
}