use std::hash::Hash;
use std::collections::HashMap;

// 缓存容量
const CACHE_SIZE:usize = 100;

// 数据项
struct Entry<K, V> {
    key:K,
    val:Option<V>,
    next:Option<usize>,
    prev:Option<usize>,
}

// LRU 缓存
struct LRU_Cache<K, V> {
    cap: usize,
    head:Option<usize>,
    tail:Option<usize>,
    map:HashMap<K, usize>,
    entries:Vec<Entry<K, V>>,
}

impl<K: Clone + Hash + Eq, V> LRU_Cache<K, V> {
    fn new() -> Self{
        Self::with_capacity(CACHE_SIZE)
    }
    
    fn with_capacity(cap:usize) -> Self{
        LRU_Cache {
            cap:cap,
            head:None,
            tail:None,
            map:HashMap::with_capacity(cap),
            entries:Vec::with_capacity(cap),
        }
    }
    
    fn len(&self) -> usize{
        self.map.len()
    }
    
    fn is_empty(&self) -> bool{
        self.map.is_empty()
    }
    
    fn is_full(&self) -> bool{
        self.map.len() >= self.cap
    }
}

impl<K: Clone + Hash + Eq, V>LRU_Cache<K, V> {
    fn insert(&mut self, key:K, val:V) -> Option<V>{
        
        // 如果想要插入的数据已经在缓存中，则更新数据，移到链表头部，并将原始值返回；
        if self.map.contains_key(&key){
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(val);
            old_val
        }else{  // 不存在键，插入
            self.ensure_room();
            
            // 更新原始头指针
            let new_index = self.entries.len();
            self.head.map(|e|{
                self.entries[e].prev = Some(new_index);
            });
            
            // 新的节点
            self.entries.push(Entry{
                key:key.clone(),
                val:Some(val),
                next:self.head,
                prev:None,
            });
            self.head = Some(new_index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, new_index);
            None
        }
    }
    
    // 确保缓存容量足够，缓存满了就移除末尾的元素
    fn ensure_room(&mut self){
        if self.is_full(){
            self.remove_tail();
        }
    }
    
    fn remove_tail(&mut self){
        if let Some(index) = self.tail{
            self.remove_from_list(index);
            if let key = self.entries[index].key.clone(){
                self.map.remove(&key);
            }
            
            if self.tail.is_none(){
                self.head = None;
            }
        }
    }
    
    fn remove_from_list(&mut self, index:usize){
        let (prev, next) = {
            let entry = self.entries.get_mut(index).unwrap();
            (entry.prev.take(), entry.next.take())
        };
        
        match(prev,next) {
            // 数据项在缓存的中间
            (Some(j),Some(k)) =>{
                let head = &mut self.entries[j];
                head.next = next;
                let next_entry = &mut self.entries[k];
                next_entry.prev = prev;
            },
            
            // 数据项在缓存的尾部
            (Some(j),None) =>{
                let head = &mut self.entries[j];
                head.next = None;
                self.tail = prev;
            },

            // 数据项在缓存的头部
            _ =>{
                if self.len() > 1{
                    let head = &mut self.entries[0];
                    head.next = None;
                    let next = &mut self.entries[1];
                    next.prev = None;
                }
            },
        }
    }
    
    // 获取某个键的值，移除原来位置的值并在头部加入
    fn access(&mut self, key:&K){
        let index = *self.map.get(key).unwrap();
        self.remove_from_list(index);
        self.head = Some(index);
    }
    
    fn get(&mut self, key:&K) -> Option<&V>{
        if self.map.contains_key(key){
            self.access(key);
        }
        
        let entries = &self.entries;
        self.map.get(key).and_then(move |&i| {
            entries[i].val.as_ref()})
    }
    
    fn remove(&mut self, key:&K) -> Option<V>{
        self.map.remove(key).and_then(|i|{
            self.remove_from_list(i);
            self.entries[i].val.take()
        })
    }

    fn contains(&mut self, key:&K)->bool{
        self.map.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut cache = LRU_Cache::with_capacity(2);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);
        cache.insert("d", 4);
        cache.insert("e", 5);

        assert!(!cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
        assert!(!cache.contains(&"c"));
        assert!(cache.contains(&"d"));
        assert!(cache.contains(&"e"));
        cache.insert("hhh", 6);
        assert!(cache.contains(&"hhh"));
    }
}