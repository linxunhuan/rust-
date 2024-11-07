// 先实现基于邻接矩阵的图， 然后实现基于邻接表的图
// 用二维矩阵来存储顶点（Vertex）和边（Edge）

use std::os::fd::OwnedFd;

// 点的定义
#[derive(Debug)]
struct Vertex<'a>{
    id:usize,
    name:&'a str,
}

impl Vertex<'_>{
    fn new(id:usize,name:&'static str)-> Self{
        Self { id, name }   
    }
}

// 边的定义
#[derive(Debug,Clone)]
struct Edge{
    edge: bool, //表示是否有边
}

impl Edge{
    fn new() -> Self{
        Self{edge:false}
    }
    
    fn set_edge() -> Self{
        Edge{edge:true}
    }
}

// 图的定义
#[derive(Debug)]
struct Graph{
    nodes:usize,
    graph: Vec<Vec<Edge>>,  // 将每个顶点的边保存到一个Vec中
}

impl Graph{
    fn new(nodes:usize) -> Self{
        Self{
            nodes,
            graph: vec![vec![Edge::new();nodes];nodes],
        }
    }
    
    fn is_empty(&self) -> bool{self.nodes == 0}
    
    fn len(&self) -> usize{self.nodes}
    
    // 添加边，设置边的属性为true
    fn add_edge(&mut self,n1:&Vertex, n2:&Vertex){
        if n1.id < self.nodes && n2.id < self.nodes{
            self.graph[n1.id][n2.id] = Edge::set_edge();
        }else{
            println!("Error, vertex beyond the graph");
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_graph(){
        let mut graph = Graph::new(4);
        let n1 = Vertex::new(0,"A");
        let n2 = Vertex::new(1,"B");
        let n3 = Vertex::new(2,"C");
        let n4 = Vertex::new(3,"D");
        graph.add_edge(&n1,&n2);
        graph.add_edge(&n2,&n3);
        println!("{:?}",graph);
        assert_eq!(graph.len(),4);
        assert_eq!(graph.is_empty(),false);
        assert_eq!(graph.graph[0][1].edge,true);
        assert_eq!(graph.graph[1][2].edge,true);
        assert_eq!(graph.graph[2][3].edge,false);
        assert_eq!(graph.graph[3][0].edge,false);
    }
}