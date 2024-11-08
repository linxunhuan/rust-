use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

// 定义用于表示颜色的枚举
// 白色——未被探索
// 灰色——正在被探索
// 黑色——已经探索完毕
#[derive(Debug, PartialEq, Clone)]
enum Color {
    White,
    Gray,
    Black,
}

// 课程节点的定义
#[derive(Debug,Clone)]
struct Vertex<T>{
    key:T,
    color:Color,
    neighbors:Vec<T>,
}

impl<T:PartialEq + Clone>Vertex<T>{
    fn new(key:T)->Self{
        Self {
            key:key,
            color:Color::White,
            neighbors:Vec::new(),
        }
    }
    
    fn add_neighbor(&mut self,neighbor:T){
        self.neighbors.push(neighbor);
    }
}

// 课程关系的定义
#[derive(Debug, Clone)]
struct Graph<T>{
    vertnums:u32,
    edgenums:u32,
    vertices:HashMap<T,Vertex<T>>,  // 所有点
    edges:HashMap<T,Vec<T>>,        // 所有边
}

impl<T:Hash + Eq + Clone + PartialEq>Graph<T> {
    fn new()->Self{
        Self {
            vertnums:0,
            edgenums:0,
            vertices:HashMap::<T,Vertex<T>>::new(),
            edges:HashMap::<T,Vec<T>>::new(),
        }
    }
    
    fn add_vertex(&mut self,key:&T)->Option<Vertex<T>>{
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(),vertex)
    }
    
    fn add_edge(&mut self,src:&T,des:&T){
        if !self.vertices.contains_key(src){
            let _ = self.add_vertex(src);
        }
        
        if !self.vertices.contains_key(des){
            let _ = self.add_vertex(des);
        }
        
        // 添加点
        self.edgenums += 1;
        self.vertices.get_mut(src).unwrap().add_neighbor(des.clone());
        // 添加边
        if !self.edges.contains_key(src){
            let _ = self.edges.insert(src.clone(),Vec::new());
        }
        self.edges.get_mut(src).unwrap().push(des.clone());
    }
}

// color    ——设置课程节点是否被访问
// schedule ——保存课程的拓扑顺序排序结果
// hascircle——控制搜索进行，遇到环就退出，说明输入数据本身有错
fn build_course_graph<T>(pre_requisties:Vec<Vec<T>>)->Graph<T>
    where T:Hash + Eq + Clone + PartialEq{
    // 为依赖的课程创建边关系
    let mut graph = Graph::new();
    for v in pre_requisties.iter(){
        let prev = v.first().unwrap();
        let last = v.last().unwrap();
        graph.add_edge(prev,last);
    }
    graph
}

// 课程规划
fn course_schedule<T>(cg:&mut Graph<T>,course:Vertex<T>,schedule:&mut Vec<String>,mut has_circle:bool)
    where T:Hash + Eq + Clone + PartialEq + Display{
    
    // 复制，防止可变引用发生冲突
    let edges = cg.edges.clone();
    
    // 对依赖课程进行探索
    let dependencies = edges.get(&course.key);
    if !dependencies.is_none(){
        for dep in dependencies.unwrap().iter(){
            let course = cg.vertices.get(dep).unwrap().clone();
            if course.color == Color::White{
                cg.vertices.get_mut(dep).unwrap().color = Color::Gray;
                course_schedule(cg,course,schedule,has_circle);
            }else if course.color == Color::Gray{
                has_circle = true;  // 遇到环就退出
                return;
            }
        }
    }
    
    // 修改节点颜色并加入schedule
    cg.vertices.get_mut(&course.key).unwrap().color = Color::Black;
    schedule.push(course.key.to_string());
}

fn find_topological_order<T>(course_num:usize,pre_requisites:Vec<Vec<T>>)
    where T:Eq + Hash + Clone + PartialEq + Display{
    
    // 构建课程关系图
    let mut cg = build_course_graph(pre_requisites);
    
    // 获取所有课程节点到courses中
    let vertices = cg.vertices.clone();
    let mut courses= Vec::new();
    for key in vertices.keys(){
        courses.push(key);
    }
    
    let mut schedule = Vec::new();  // 保存可行的课程安排
    let mut has_circle = false;             // 是否有环
    
    // 对课程进行拓扑排序
    for num in 0..course_num{
        let course = cg.vertices.get(&courses[num]).unwrap().clone();
        
        // 仅当无环且课程节点未被探索时才进行下一步探索
        if course.color == Color::White && !has_circle{
            // 修改课程节点的颜色，表示当前节点正在被探索
            cg.vertices.get_mut(&courses[num]).unwrap().color = Color::Gray;
            course_schedule(&mut cg,course,&mut schedule,has_circle);
        }
    }
    
    if !has_circle{
        println!("{:#?}",schedule);
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_find_top(){
        let course_num = 7;
        
        // 构建课程的依赖关系
        let mut pre_requisites = Vec::<Vec<&str>>::new();
        pre_requisites.push(vec!["微积分","函数"]);
        pre_requisites.push(vec!["微积分","导数"]);
        pre_requisites.push(vec!["线性代数","方程组"]);
        pre_requisites.push(vec!["卷积网络","微积分"]);
        pre_requisites.push(vec!["卷积网络","概率论"]);
        pre_requisites.push(vec!["卷积网络","线性代数"]);
        
        // 找到拓扑排序结果，此为合理的课程学习顺序
        find_topological_order(course_num,pre_requisites);
    }
}
