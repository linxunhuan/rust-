use std::rc::Rc;
use std::cell::RefCell;

// 链接的定义
type Link = Option<Rc<RefCell<Node>>>;

// 节点的定义
struct Node {
    data:usize,
    next:Link,
}

impl Node {
    fn new(data:usize) -> Self{
        Self {data:data, next:None}
    }
}

// 图的定义
struct Graph {
    first:Link,
    last:Link,
}

impl Graph {
    fn new() -> Self{
        Self {first:None, last:None}
    }
    
    fn is_empty(&self) -> bool{
        self.first.is_none()
    }
    
    fn get_first(&self) -> Link{
        self.first.clone()
    }
    
    fn print_nodes(&self){
        let mut node = self.get_first();
        while let Some(n) = node{
            print!("{}", n.borrow().data);
            node = n.borrow().next.clone();
        }
        println!("\n");
    }
    
    fn insert(&mut self, data:usize){
        let node = Rc::new(RefCell::new(Node::new(data)));
        if self.is_empty(){
            self.first = Some(node.clone());
            self.last = Some(node);
        }else { 
            self.last.as_mut().unwrap().borrow_mut().next = Some(node.clone());
            self.last = Some(node);
        }
    }
}

// 构建图
fn build_graph(data:[[usize;2];20]) -> Vec<(Graph,usize)>{
    let mut graph_list:Vec<(Graph,usize)> = Vec::new();
    for _ in 0..9{
        graph_list.push((Graph::new(),0));
    }
    
    for i in 1..9{
        for j in 0..data.len(){
            if data[j][0] == i{
                graph_list[i].0.insert(data[j][1]);
            }
        }
        print!("[{i}]->");
        graph_list[i].0.print_nodes();
    }
    graph_list
} 

fn dfs(graph_list:&mut Vec<(Graph,usize)>){
    let mut graph_nodes:Vec<usize> = Vec::new();
    let mut temp_stack:Vec<usize> = Vec::new();
    
    graph_list[1].1 = 1;
    let mut current_node = graph_list[1].0.get_first().clone();
    
    // 输出图
    print!("[1]->");
    while let Some(val) = current_node {
        graph_nodes.insert(0, val.borrow().data);
        current_node = val.borrow().next.clone(); 
    }
    
    // 输出深度优先遍历结果
    loop{
        if graph_nodes.is_empty(){
            break;
        }else { 
            let data = graph_nodes.pop().unwrap();
            // 未被访问
            if graph_list[data].1 == 0{
                // 更改访问状态为已访问过
                graph_list[data].1 = 1;
                print!("{data}->");
                
                // 将节点添加到temp中并对其进行深度优先遍历
                let mut node = graph_list[data].0.get_first().clone();
                while let Some(val) = node {
                    temp_stack.push(val.borrow().data);
                    node = val.borrow().next.clone();
                }

                while !temp_stack.is_empty() {
                    graph_nodes.push(temp_stack.pop().unwrap());
                }
            }
        }
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    
}