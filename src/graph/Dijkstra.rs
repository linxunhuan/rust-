use std::cmp::Ordering;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::hash::Hash;

// 点的定义
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Vertex<'a>{
    name:&'a str,
}

impl <'a> Vertex<'a>{
    fn new(name:&'a str)->Vertex<'a>{
        Vertex { name }
    }
}

// 访问过的点
#[derive(Debug)]
struct Visited<T>{
    vertex:T,
    distance:usize,
}

// 为visited 添加比较功能
impl<T> Ord for Visited<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T>PartialOrd for Visited<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T>Eq for Visited<T> {}
impl<T>PartialEq for Visited<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

// 最短路径算法
fn dijkstra<'a>(start:Vertex<'a>, adj_list:&HashMap<Vertex<'a>,Vec<(Vertex<'a>,usize)>>)->HashMap<Vertex<'a>,usize> {
    let mut distances = HashMap::new();  // 距离
    let mut visited: HashSet<Vertex<'a>>= HashSet::new();   // 访问过的点
    let mut heap = BinaryHeap::new();      // 待访问的点
    
    // 设置起始点和距离个点的初始位置
    distances.insert(start,0);
    heap.push(Visited{vertex:start,distance:0});
    
    while let Some(Visited{vertex,distance}) = heap.pop(){
        // 已经访问过这个点，继续下一个点
        if visited.contains(&vertex){
            continue;
        }
        
        // 获取邻接点
        if let Some(nbrs) = adj_list.get(&vertex){
            for (nbr, weight) in nbrs{
                let new_distance = distance + weight;
                let is_shorter = distances.get(&nbr)
                                                .map_or(true,|&current| new_distance < current);
                
                // 如果新的距离更短，则更新距离和路径
                if is_shorter{
                    distances.insert(*nbr,new_distance);
                    heap.push(Visited{
                            vertex:*nbr,
                            distance:new_distance
                        });
                }
            }
        }
    }
    
    
    distances
}



#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_dijkstra() {
        let v1 = Vertex::new("v1");
        let v2 = Vertex::new("v2");
        let v3 = Vertex::new("v3");
        let v4 = Vertex::new("v4");
        let v5 = Vertex::new("v5");
        let v6 = Vertex::new("v6");


        let mut adj_list = HashMap::new();
        adj_list.insert(v1, vec![(v2, 1), (v3, 4)]);
        adj_list.insert(v2, vec![(v4, 5)]);
        adj_list.insert(v3, vec![(v2, 2), (v6, 2)]);
        adj_list.insert(v4, vec![(v5, 3)]);
        adj_list.insert(v5, vec![]);
        adj_list.insert(v6, vec![(v5, 1)]);

        // 求点v1到其他所有点的最短路径
        let distances = dijkstra(v1, &adj_list);
        for (v, d) in &distances {
            println!("{}-{},最短距离为{d}", v1.name, v.name);
        }
    }
}