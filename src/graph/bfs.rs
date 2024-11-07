// 实现广度优先搜索
use std::rc::Rc;
use std::cell::RefCell;

// 因为节点之间存在多个共享的链接，所以需要Rc
// 又因为Rc不可变，所以使用具有内部可变性的RefCell进行包裹

type Link = Option<Rc<RefCell<Node>>>;

// 节点的定义
struct Node{
    data:usize,
    next: Link,
}

impl Node{
    fn new(data: usize) -> Self{
        Self { data:data, next: None }
    }
}

// 图的定义及实现
struct Graph{
    first: Link,
    last: Link,
}

impl Graph{
    fn new() -> Self{
        Self { first: None, last: None }
    }

    fn is_empty(&self) -> bool{
        self.first.is_none()
    }

    fn get_first(&self) -> Link{
        self.first.clone()
    }

    // 输出节点
    fn print_node(&self){
        let mut curr = self.first.clone();
        while let Some(val) = curr{
            print!("[{}]", &val.borrow().data);
            curr = val.borrow().next.clone();
        }
        print!("\n");
    }

    // 插入节点
    fn insert(&mut self, data: usize){
        let node = Rc::new(RefCell::new(Node::new(data)));

        if self.is_empty(){
            self.first = Some(node.clone());
            self.last = Some(node.clone());
        }else{
            self.last.as_mut().unwrap()
                .borrow_mut().next = Some(node.clone());
            self.last = Some(node);
        }
    }
}

// 该函数用于构建data图，并将图封装成元组，然后保存到vec中
// 元组中的第二个值用于表示节点是否被访问过，0表示未访问过，1表示访问过
fn build_graph(data: [[usize;2];24]) -> Vec<(Graph, usize)>{
    let mut graphs: Vec<(Graph, usize)> = Vec::new();
    for _ in 0..9{graphs.push((Graph::new(), 0));}
    for i in 0..9{
        for j in 0..data.len(){
            if data[j][0] == i{
                graphs[i].0.insert(data[j][1]);
            }
        }
        print!("[{i}]->");
        graphs[i].0.print_node();
    }
    graphs
}

fn bfs(graph: Vec<(Graph, usize)>){
    let mut gp = graph;
    let mut nodes = Vec::new();
    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 输出图
    print!("图[1]->：");
    while let Some(val) = curr{
        nodes.push(val.borrow().data);
        curr = val.borrow().next.clone();
    }

    // 输出广度优先图
    loop{
        if nodes.len() == 0{
            break;
        }else {
            // nodes中的首节点被弹出，这里模仿了队列的特性
            let node = nodes.remove(0);

            // 节点未被访问过，加入nodes，修改其访问状态为1
            if gp[node].1 == 0{
                gp[node].1 = 1;

                // 输出当前节点值
                print!("{}->",node);

                // 将与当前节点相连的节点加入nodes
                let mut curr = gp[node].0.get_first().clone();
                
                while let Some(val) = curr{
                    nodes.push(val.borrow().data);
                    curr = val.borrow().next.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_bfs(){
        let data = [
            [1,2],[2,1],[1,3],[3,1],[2,4],[4,2],
            [2,5],[5,2],[3,6],[6,3],[3,7],[7,3],
            [4,8],[8,4],[5,9],[9,5],[6,10],[10,6],
            [7,11],[11,7],[8,12],[12,8],[9,13],[13,9],
        ];
        let gp = build_graph(data);
        bfs(gp);
    }
}