use std::{hash::Hash, collections::{ HashMap}};

// 点的定义
#[derive(Debug, Clone)]
struct Vertex<T> {
    key:T,
    neighbors: Vec<(T,i32)>,// 邻接点的集合
}

impl<T:Clone + PartialEq> Vertex<T> {
    fn new(key:T) -> Self {
        Vertex {
            key,
            neighbors: Vec::new(),
        }
    }
    fn add_neighbor(&mut self, neighbor:T, weight:i32) {
        self.neighbors.push((neighbor, weight));
    }
    
    // 判断与当前顶点是否相邻
    fn adjacent_key(&self, key:&T) -> bool {
        for(nbr, _) in self.neighbors.iter() {
            if nbr == key { return true;}
        }
        false
    }
    
    // 获取与当前顶点相邻的顶点
    fn get_neighbors(&self) -> Vec<T> {
        let mut nbrs = Vec::new();
        for(nbr, _) in self.neighbors.iter() {
            nbrs.push(nbr.clone());
        }
        nbrs
    }
    
    // 获取到邻接点的权重
    fn get_weight(&self, key:&T) -> &i32 {
        for(nbr, wt) in self.neighbors.iter() {
            if nbr == key { return wt;}
        }
        &0
    }
}

// 图的定义
#[derive(Debug, Clone)]
struct Graph<T>{
    vertnums:u32,                   // 顶点数量
    edgenums:u32,                   // 边数量
    vertices:HashMap<T, Vertex<T>>, // 顶点的集合
}

impl<T:Clone + Hash + Eq + PartialEq>Graph<T> {
    fn new() -> Self {
        Graph {
            vertnums:0,
            edgenums:0,
            vertices:HashMap::<T, Vertex<T>>::new(),
        }
    }
    
    fn is_empty(&self) -> bool { self.vertnums == 0 }
    
    fn vertex_nums(&self) -> u32 { self.vertnums }
    
    fn edge_nums(&self) -> u32 { self.edgenums }
    
    fn contains(&self, key:&T) -> bool {
        for(nbr, _) in self.vertices.iter() {
            if nbr == key { return true;}
        }
        false
    }
    
    fn add_vertex(&mut self, key:T)->Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }
    
    fn get_vertex(&self, key:&T) -> Option<&Vertex<T>> {
        if let Some(vertex) = self.vertices.get(key) {
            return Some(vertex);
        }else { 
            None
        }
    }
    
    // 获取所有顶点的键
    fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.vertices.keys() {
            keys.push(key.clone());
        }
        keys
    }
    
    // 删除顶点（同时也要删除边）
    fn remove_vertex(&mut self, key:&T) -> Option<Vertex<T>> {
        
        // 从图中移除给定键对应的顶点，并减少顶点计数
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;
        
        // 删除从当前顶点出发的所有边
        for vertex in self.vertex_keys() {
            
            // 删除从当前顶点出发的所有边
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                // 获取每个顶点的可变引用
                if vt.adjacent_key(key){
                    
                    // 保留不与待删除顶点相邻的边，删除与其相邻的边
                    /**neighbors 是一个 HashMap，其键是 T 类型，值是 i32 类型
                    闭包的参数 k 实际上是一个元组 (key, value)，其中 key 是 T 类型，value 是 i32 类型
                    闭包参数 (k, _) 将 neighbors 中的每个元素（一个元组 (key, value)）展开为 k 和 _（忽略值 value），只比较 key 部分
                    */
                    vt.neighbors.retain(|(k, _)| k != key);
                    // 减少边的计数
                    self.edgenums -= 1;
                }
            }
        }
        old_vertex
    }
    
    fn add_edge(&mut self, start:&T, end:&T, weight:i32) {
        // 若顶点不存在，则需要先添加顶点
        if !self.contains(start) {
            self.add_vertex(start.clone());
        }
        
        if !self.contains(end) {
            self.add_vertex(end.clone());
        }
        
        // 添加边
        self.edgenums += 1;
        self.vertices.get_mut(start).unwrap()
            .add_neighbor(end.clone(), weight);
    }
    
    // 判断两个顶点是否相邻
    fn adjacent(&self, start:&T, end:&T) -> bool {
        self.vertices.get(start).unwrap().adjacent_key(end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_graph() {
        let mut g = Graph::new();
        
        for i in 0..6 {g.add_vertex(i);}
        println!("graph: {:?}", g.is_empty());
        
        let vertices = g.vertex_keys();
        for vtx in vertices {
            println!("Vertex:{:#?}", vtx);
        }
        
        g.add_edge(&0, &1, 5);
        g.add_edge(&0, &5, 2);
        g.add_edge(&1, &2, 4);
        g.add_edge(&2, &3, 9);
        g.add_edge(&3, &4, 7);
        g.add_edge(&3, &5, 3);
        g.add_edge(&4, &0, 1);
        g.add_edge(&4, &4, 8);
        println!("vert nums: {}",g.vertex_nums());
        println!("edge nums: {}",g.edge_nums());
        println!("contains 0:{}",g.contains(&0));
        
        let vertex = g.get_vertex(&0).unwrap();
        println!("key:{},to nbr 1 weight:{}", vertex.key,vertex.get_weight(&1));
        
    }
}
