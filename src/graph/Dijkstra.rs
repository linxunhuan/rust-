use std::cmp::Ordering;
use std::collections::{BinaryHeap,HashMap,HashSet};

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

// 为访问过的点添加全序比较功能


// 最短路径算法




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