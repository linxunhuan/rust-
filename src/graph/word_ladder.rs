use std::{hash::Hash, collections::{HashMap}};
use crate::structure::queue::Queue;
// 有了BFS算法，接下来研究如何使用广度优先搜索解决最短转换路径问题
// 所以还需要加入distance参数

#[derive(Clone,Debug,PartialEq)]
enum Color{
    White,  // 未被探索
    Gray,   // 正在被探索
    Black,  // 已经被探索
}

// 节点的定义
#[derive(Clone,Debug)]
struct Vertex<T>{
    color: Color,
    distance: u32,  // 与起始点的最短距离为最少转换次数
    key:T,
    neighbors: Vec<(T,u32)>,
}

impl<T:Clone + PartialEq>Vertex<T> {
    fn new(key:T) -> Self{
        Self{
            color: Color::White,
            distance: 0,
            key:key,
            neighbors: Vec::new(),
        }
    }
    
    fn add_neighbor(&mut self,neighbors:T,weight:u32){
        self.neighbors.push((neighbors,weight));
    }
    
    // 获取邻接节点
    fn get_neighbors(&self) -> Vec<&T>{
        let mut neighbors = Vec::new();
        for(nbr,_)in self.neighbors.iter(){
            neighbors.push(nbr);
        }
        neighbors
    }
}

// 图的定义
#[derive(Debug,Clone)]
struct Graph<T>{
    vertnums:u32,
    edgenums:u32,
    vertices:HashMap<T,Vertex<T>>,
}

impl<T:Clone + Hash + Eq + PartialEq> Graph<T>{
    fn new() -> Self{
        Self{
            vertnums:0,
            edgenums:0,
            vertices:HashMap::<T, Vertex<T>>::new(),
        }
    }
    
    fn contains(&self,key:&T) -> bool{
        for(nbr,_) in self.vertices.iter(){
            if nbr == key{return true;}
        }
        false
    }
    
    // 添加节点
    fn add_vertex(&mut self,key:&T) -> Option<Vertex<T>>{
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(),vertex)
    }
    
    // 添加边
    fn add_edge(&mut self,from:&T,to:&T,weight:u32){
        // 节点如果不存在，则需要先添加节点
        if !self.contains(from){
            let _ = self.add_vertex(from);
        }
        if !self.contains(to){
            let _ = self.add_vertex(to);
        }
        
        self.edgenums += 1;
        self.vertices.get_mut(from).unwrap().add_neighbor(to.clone(),weight);
    }
}

fn build_word_graph(words:Vec<&str>) -> Graph<String>{
    let mut hmap:HashMap<String,Vec<String>> = HashMap::new();
    
    // 构建单词- 模式 HashMap
    for word in words.iter(){
        for i in 0..word.len(){
            let pattern = word[..i].to_string() + "_" + &word[i+1..];
            if hmap.contains_key(&pattern){
                hmap.get_mut(&pattern).unwrap().push(word.to_string());
            }else { 
                hmap.insert(pattern,vec![word.to_string()]);
            }
        }
    }
    
    // 双向连接图，彼此距离为1
    let mut graph = Graph::<String>::new();
    for word in hmap.keys(){
        for w1 in &hmap[word]{
            for w2 in &hmap[word]{
                if w1 != w2{
                    graph.add_edge(w1,w2,1);
                }
            }
        }
    }
    graph
}

// 下面基于BFS算法原理进行最短路径的搜索
// 虽然定义了三种颜色，但实际上只有白色节点才会入队
// 所以灰色节点既可以置为黑色，也可以不置为黑色

// 字梯图-广度优先搜索
fn word_ladder(g:&mut Graph<String>,start:Vertex<String>,end:Vertex<String>, len:usize) -> u32{
    // 判断起始点是否存在
    if !g.vertices.contains_key(&start.key){return 0;}
    if !g.vertices.contains_key(&end.key){return 0;}
    
    // 准备队列，加入起始点
    let mut vertex_queue = Queue::new(len);
    let _ = vertex_queue.enqueue(start);
    
    while vertex_queue.len() > 0{
        // 节点出队
        let curr = vertex_queue.dequeue().unwrap();
        for nbr in curr.get_neighbors(){
            // 复制，以免和图中节点冲突
            // 如果节点是RefCell包裹的，则不需要复制
            let mut nbv = g.vertices.get(nbr).unwrap().clone();
            if end.key != nbv.key{
                // 只有白色节点才可以入队，其他颜色都处理过了
                if Color::White == nbv.color{
                    // 更新节点颜色和距离并加入队列
                    nbv.color = Color::Gray;
                    nbv.distance = curr.distance + 1;
                    
                    // 图中的节点也需要更新颜色和距离
                    g.vertices.get_mut(nbr).unwrap().color = Color::Gray;
                    g.vertices.get_mut(nbr).unwrap().distance = curr.distance + 1;
                    
                    // 白色的节点加入队列
                    let _ = vertex_queue.enqueue(nbv);
                }
                // 其他颜色不需要处理，两种颜色足以
            }else { 
                // curr 的邻接节点里有end，再转换一次就够了
                return curr.distance + 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_word_ladder(){
        let words = vec![
            "FOOL","COOL","POOL","FOUL","FOIL","FAIL","FALL",
            "POLL","PALL","POLE","PALE","SALE","PAGE","SAGE",
        ];
        
        let len = words.len();
        let mut graph = build_word_graph(words);
        
        graph.vertices.get_mut("FOOL").unwrap().clone();
        
        let start = graph.vertices.get("FOOL").unwrap().clone();
        let end = graph.vertices.get("SAGE").unwrap().clone();
        
        let distance = word_ladder(&mut graph,start,end,len);
        println!("距离为：{}",distance);
        
    }
}