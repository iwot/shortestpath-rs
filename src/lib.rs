//! # 最短経路探索
//! 
//! `shortestpath`は、最短経路探索を行うためのライブラリです。
use std::collections::HashMap;

pub type GraphIndex = String;

#[derive(Debug)]
pub struct Node {
    name: GraphIndex,
    done: bool,
    edges: Vec<Edge>,
    costed: i32,
    prev: Option<GraphIndex>,
    passage: Option<Edge>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    next: GraphIndex,
    name: String,
    cost: i32,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<GraphIndex, Node>
}

pub fn new_graph() -> Graph {
    Graph {nodes: HashMap::new()}
}

impl Graph {
    /// グラフにノード(src)とノード(dst)の繋がり（辺(edge_name)）を追加する。
    /// 
    /// #Examples
    /// 
    /// ```
    /// use shortestpath::new_graph;
    /// let mut g = new_graph();
    /// 
    /// g.add("s", "a", 2, "edge1");
    /// ```
    pub fn add<'a>(&mut self, src: &'a str, dst: &'a str, cost: i32, edge_name: &'a str) {
        self.nodes.entry(dst.to_string()).or_insert(Node {
            name: dst.to_string(),
            done: false,
            edges: vec![],
            costed: -1,
            prev: Some(src.to_string()),
            passage: None,
        });

        let node = self.nodes.entry(src.to_string()).or_insert(Node {
            name: src.to_string(),
            done: false,
            edges: vec![],
            costed: -1,
            prev: None,
            passage: None,
        });

        let edge = Edge{next:dst.to_string(), name: edge_name.to_string(), cost: cost};
        node.edges.push(edge);
    }

    fn node_prev<'a>(&self, name: &'a str) -> Option<String> {
        if let Some(node) = self.nodes.get(name) {
            node.prev.clone()
        } else {
            None
        }
    }

    fn node_costed<'a>(&self, name: &'a str) -> i32 {
        if let Some(node) = self.nodes.get(name) {
            node.costed
        } else {
            -1
        }
    }

    fn node_edges<'a>(&self, name: &'a str) -> Vec<Edge> {
        if let Some(node) = self.nodes.get(name) {
            node.edges.clone()
        } else {
            vec![]
        }
    }

    fn is_done_node<'a>(&self, name: &'a str) -> bool {
        if let Some(node) = self.nodes.get(name) {
            node.done
        } else {
            false
        }
    }

    fn update_node_edge<'a>(&mut self, next_node_name: &'a str, cost: i32, done_node_name: &'a str, passed_edge: Edge) {
        if let Some(node) = self.nodes.get_mut(next_node_name) {
            node.costed = cost;
            node.prev = Some(done_node_name.to_string());
            node.passage = Some(passed_edge);
        }
    }

    fn update_node_done<'a>(&mut self, node_name: &'a str, done: bool) {
        if let Some(node) = self.nodes.get_mut(node_name) {
            node.done = done;
        }
    }

    /// 開始ノードから終了ノードまでの最短経路探索を行い、結果をShortestPath型で返します。
    /// 
    /// # Examples
    /// 
    /// ```
    /// use shortestpath::new_graph;
    /// let mut g = new_graph();
    /// g.add("s", "a", 2, "edge1");
    /// g.add("s", "b", 5, "edge2");
    /// g.add("a", "b", 2, "edge3");
    /// g.add("a", "c", 5, "edge4");
    /// g.add("b", "c", 4, "edge5");
    /// g.add("b", "d", 2, "edge6");
    /// g.add("c", "z", 7, "edge7");
    /// g.add("d", "c", 5, "edge8");
    /// g.add("d", "z", 2, "edge9");
    /// let result = g.shortest_path("s", "z");
    /// ```
    pub fn shortest_path<'a>(&mut self, start: &'a str, goal: &'a str) -> ShortestPath {
        if let Some(start_node) = self.nodes.get_mut(start) {
            start_node.costed = 0;

            loop {
                let mut done_node : Option<String> = None;

                for (name, node) in &self.nodes {
                    if node.done || node.costed < 0 {
                        continue;
                    }

                    if done_node.is_none() {
                        done_node = Some(name.to_string());
                    } else if node.costed < self.node_costed(done_node.clone().unwrap().as_ref()) {
                        done_node = Some(name.to_string());
                    }
                }

                if done_node.is_none() {
                    break;
                }

                let done_node_name = done_node.unwrap();

                for edge in self.node_edges(&done_node_name.clone()) {
                    let passed_edge = edge.clone();
                    let next_node = edge.next;

                    if self.is_done_node(next_node.as_ref()) {
                        continue;
                    }

                    let new_cost = self.node_costed(&done_node_name.clone()) + edge.cost;
                    let next_node_costed = self.node_costed(&next_node.clone());
                    if next_node_costed == -1 || new_cost < next_node_costed {
                        self.update_node_edge(&next_node, new_cost, &done_node_name, passed_edge);
                    }
                }

                self.update_node_done(&done_node_name, true);
                
                if done_node_name == goal {
                    break;
                }
            }
            
            let mut passages = vec![];
            let mut node_name = goal.to_string();
            loop {
                if let Some(node) = self.nodes.get(&node_name) {
                    passages.push(WayKind::Node(node.name.clone()));
                    
                    if let Some(ref passed_edge) = node.passage {
                        passages.push(WayKind::Edge(passed_edge.name.clone(), passed_edge.cost));
                    }
                    
                    if node.name == start {
                        break;
                    }
                    node_name = self.node_prev(&node_name).unwrap();
                } else {
                    break;
                }
            }

            passages.reverse();
            ShortestPath {passages: passages, total_cost: self.nodes.get(goal).unwrap().costed}
        } else {
            ShortestPath {passages: vec![], total_cost: -1}
        }
    }
}

#[derive(Debug)]
pub struct ShortestPath {
    passages: Vec<WayKind>,
    total_cost: i32,
}

#[derive(Debug)]
pub enum WayKind {
    Node(String),
    Edge(String, i32),
    None,
}

impl ShortestPath {
    pub fn get_node_path(&self) -> &Vec<WayKind> {
        &self.passages
    }

    pub fn get_node_path_string<'a>(&self, connector: &'a str) -> String {
        let mut stock = vec![];
        for s in &self.passages {
            if let WayKind::Node(ref node_name) = s {
                stock.push(node_name.clone());
            }
        }
        stock.join(connector)
    }

    pub fn get_node_edge_path_string<'a>(&self, connector: &'a str) -> String {
        let mut stock = vec![];
        for s in &self.passages {
            if let WayKind::Edge(ref edge_name, _) = s {
                stock.push(format!("(({}))", edge_name));
            }
            if let WayKind::Node(ref node_name) = s {
                stock.push(node_name.clone());
            }
        }
        stock.join(connector)
    }

    pub fn cost(&self) -> i32 {
        self.total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_success() {
        let mut g = new_graph();
        g.add("s", "a", 2, "edge1");
        g.add("s", "b", 5, "edge2");
        g.add("a", "b", 2, "edge3");
        g.add("a", "c", 5, "edge4");
        g.add("b", "c", 4, "edge5");
        g.add("b", "d", 2, "edge6");
        g.add("c", "z", 7, "edge7");
        g.add("d", "c", 5, "edge8");
        g.add("d", "z", 2, "edge9");
        let result = g.shortest_path("s", "z");

        assert_eq!(8, result.cost());
        assert_eq!("s->a->b->d->z", result.get_node_path_string("->"));
        assert_eq!("s->((edge1))->a->((edge3))->b->((edge6))->d->((edge9))->z", result.get_node_edge_path_string("->"));
        
        // let result = result.get_node_path();
        // dbg!(result);
    }
}
