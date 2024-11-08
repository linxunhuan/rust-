use std::collections::HashMap;
use std::hash::Hash;
use crate::structure::queue::Queue;

// 定义用于表示颜色的枚举
#[derive(Debug,Clone, PartialEq)]
enum Color {
    White,  // 白色
    Gray,   // 灰色
}

// 定义城市节点
#[derive(Debug,Clone)]
struct Vertex<T> {
    key: T,
    color: Color,
    neighbors: Vec<T>,
}

impl<T:PartialEq + Clone>Vertex<T>{
    fn new(key:T) -> Self{
        Self{
            key:key,
            color:Color::White,
            neighbors:Vec::new(),
        }
    }
    
    fn add_neighbor(&mut self,neighbor:T){
        self.neighbors.push(neighbor);
    }
    
    fn get_neighbors(&self)->Vec<&T>{
        let mut neighbors=Vec::new();
        for nbr in self.neighbors.iter(){
            neighbors.push(nbr);
        }
        neighbors
    }
}

// 定义省份图
#[derive(Debug,Clone)]
struct Graph<T>{
    vertnums:u32,
    edgenums:u32,
    vertices:HashMap<T,Vertex<T>>,
    edges:HashMap<T,Vec<T>>,
}

impl<T:Eq + PartialEq + Hash + Clone>Graph<T>{
    fn new() -> Self{
        Self{
            vertnums:0,
            edgenums:0,
            vertices:HashMap::<T,Vertex<T>>::new(),
            edges:HashMap::<T,Vec<T>>::new(),
        }
    }
    
    fn add_vertex(&mut self,key:T) ->Option<Vertex<T>>{
        let vertex=Vertex::new(key.clone());
        self.vertnums +=1;
        self.vertices.insert(key.clone(),vertex)
    }
    
    fn add_edge(&mut self,from:&T,to:&T){
        if !self.vertices.contains_key(from){
            let _ = self.add_vertex(from.clone());
        }
        
        if !self.vertices.contains_key(to){
            let _ = self.add_vertex(to.clone());
        }
        
        // 添加点
        self.edgenums +=1;
        self.vertices.get_mut(from).unwrap().add_neighbor(to.clone());
        
        // 添加边
        if !self.edges.contains_key(from){
            let _ = self.edges.insert(from.clone(),Vec::new());
        }
        self.edges.get_mut(from).unwrap().push(to.clone());
    }
}

// 构建城市连接关系图
fn build_city_graph<T>(connected:Vec<Vec<T>>) -> Graph<T>
    where T:Eq + PartialEq + Hash + Clone{
    // 在有关联关系的城市节点之间设置边
    let mut city_graph=Graph::<T>::new();
    for val in connected.iter(){
        let from=val.first().unwrap();
        let to=val.last().unwrap();
        city_graph.add_edge(&from,&to);
    }
    city_graph
}

fn find_province_num_bfs<T>(connected:Vec<Vec<T>>) -> u32
    where T:Eq + PartialEq + Hash + Clone{
    // 初始化颜色数组
    let mut color_array=build_city_graph(connected);
    
    // 获取各个主节点城市的键
    let mut cities = Vec::new();
    for key in color_array.edges.keys(){
        cities.push(key.clone());
    }
    
    // 逐个处理强连通分量
    let mut province_num=0;
    let mut queue = Queue::new(cities.len());
    for ct in &cities{
        let city = color_array.vertices.get(ct).unwrap().clone();
        if city.color==Color::White{
            
            // 改变当前节点的颜色并入队
            color_array.vertices.get_mut(ct).unwrap().color=Color::Gray;
            queue.enqueue(city);
            while !queue.is_empty(){
                
                // 获取某个节点及其相邻节点
                let queue_city = queue.dequeue().unwrap();
                let neighbors = queue_city.get_neighbors();
                
                // 逐个处理相邻节点
                for nbr in neighbors{
                    let neighbor_city = color_array.vertices.get(nbr).unwrap().clone();
                    if neighbor_city.color==Color::White{
                        
                        // 当前节点的相邻节点未被访问过，将其颜色设置为灰色，并将其加入队列
                        color_array.vertices.get_mut(nbr).unwrap().color=Color::Gray;
                        queue.enqueue(neighbor_city);
                    }
                }
            }
            // 处理完一个强连通分量后
            province_num +=1;
        }
    }
    province_num
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        
        // 构建城市依赖关系
        let mut connected=Vec::<Vec<&str>>::new();
        connected.push(vec!["成都","西华"]);
        connected.push(vec!["成都","内江"]);
        connected.push(vec!["成都","德阳"]);
        connected.push(vec!["成都","乐山"]);
        connected.push(vec!["成都","攀枝花"]);
        connected.push(vec!["成都","自贡"]);
        connected.push(vec!["成都","泸州"]);
        connected.push(vec!["成都","宜宾"]);
        connected.push(vec!["自贡","成都"]);
        
        connected.push(vec!["郑州","洛阳"]);
        connected.push(vec!["郑州","开封"]);
        connected.push(vec!["郑州","信阳"]);
        connected.push(vec!["郑州","南阳"]);
        connected.push(vec!["郑州","周口"]);
        connected.push(vec!["郑州","驻马店"]);
        connected.push(vec!["郑州","三门峡"]);
        connected.push(vec!["洛阳","郑州"]);

        connected.push(vec!["广州","深圳"]);
        connected.push(vec!["广州","东莞"]);
        connected.push(vec!["广州","珠海"]);
        connected.push(vec!["广州","汕头"]);
        connected.push(vec!["广州","佛山"]);
        connected.push(vec!["广州","江门"]);
        connected.push(vec!["广州","湛江"]);
        connected.push(vec!["深圳","广州"]);
        
        // 找到所有的强联通分量
        let province_num=find_province_num_bfs(connected);
        println!("省份数量为：{}",province_num);
        
        
    }
}