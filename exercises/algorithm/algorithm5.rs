#![allow(unused)] 

/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/



use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, // 这就是一个邻接表的实现方式
}

impl Graph {

    // Create a new graph with n vertices
    // 建立一个邻接表
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }


    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // BFS 从设定的初始节点入队，只要队列不空，就一直找邻接的顶点
        
        let mut q=VecDeque::new();
        let mut visit_order = vec![];
        q.push_back(start); // 入队（后进）
        // 队不空
        while !q.is_empty(){
            // 将队列当中的元素出队（前出）
            if let Some(u) = q.pop_front(){
                visit_order.push(u); // 代表访问过了
                // 找和 u 临接的顶点
                for ele in self.adj[u].iter(){
                    // 已经访问过的顶点当中没有，且队列当中没有（我觉得这里能优化的起始）
                    if !visit_order.contains(ele) && !q.contains(ele){
                        q.push_back(*ele); 
                    }
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}


fn main(){}