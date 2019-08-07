use std::collections::HashMap;

pub type GraphIndex = String;

#[derive(Debug)]
pub struct Node {
    name: GraphIndex,
    done: bool,
    edges: Vec<Edge>,
    costed: i32,
    prev: Option<GraphIndex>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    next: GraphIndex,
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
    pub fn add<'a>(&mut self, src: &'a str, dst: &'a str, cost: i32) {
        self.nodes.entry(dst.to_string()).or_insert(Node {
            name: dst.to_string(),
            done: false,
            edges: vec![],
            costed: -1,
            prev: Some(src.to_string()),
        });

        let node = self.nodes.entry(src.to_string()).or_insert(Node {
            name: src.to_string(),
            done: false,
            edges: vec![],
            costed: -1,
            prev: None,
        });

        let edge = Edge{next:dst.to_string(), cost: cost};
        node.edges.push(edge);
    }

    pub fn node_prev<'a>(&self, name: &'a str) -> Option<String> {
        if let Some(node) = self.nodes.get(name) {
            node.prev.clone()
        } else {
            None
        }
    }

    pub fn node_costed<'a>(&self, name: &'a str) -> i32 {
        if let Some(node) = self.nodes.get(name) {
            node.costed
        } else {
            -1
        }
    }

    pub fn node_edges<'a>(&self, name: &'a str) -> Vec<Edge> {
        if let Some(node) = self.nodes.get(name) {
            node.edges.clone()
        } else {
            vec![]
        }
    }

    pub fn is_done_node<'a>(&self, name: &'a str) -> bool {
        if let Some(node) = self.nodes.get(name) {
            node.done
        } else {
            false
        }
    }

    pub fn update_node_edge<'a>(&mut self, next_node_name: &'a str, cost: i32, done_node_name: &'a str) {
        if let Some(node) = self.nodes.get_mut(next_node_name) {
            node.costed = cost;
            node.prev = Some(done_node_name.to_string());
        }
    }

    pub fn update_node_done<'a>(&mut self, node_name: &'a str, done: bool) {
        if let Some(node) = self.nodes.get_mut(node_name) {
            node.done = done;
        }
    }

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
                    let next_node = edge.next;

                    if self.is_done_node(next_node.as_ref()) {
                        continue;
                    }

                    let new_cost = self.node_costed(&done_node_name.clone()) + edge.cost;
                    let next_node_costed = self.node_costed(&next_node.clone());
                    if next_node_costed == -1 || new_cost < next_node_costed {
                        self.update_node_edge(&next_node, new_cost, &done_node_name);
                    }
                }

                self.update_node_done(&done_node_name, true);
                
                if done_node_name == goal {
                    break;
                }
            }
            
            let mut result_nodes = vec![];
            let mut node_name = goal.to_string();
            loop {
                if let Some(node) = self.nodes.get(&node_name) {
                    result_nodes.push(ShortestPathNode{
                        name: node.name.clone(),
                        piled_cost: self.node_costed(&node.name.clone()),
                    });
                    if node.name == start {
                        break;
                    }
                    node_name = self.node_prev(&node_name).unwrap();
                } else {
                    break;
                }
            }

            result_nodes.reverse();
            ShortestPath {nodes: result_nodes, total_cost: self.nodes.get(goal).unwrap().costed}
        } else {
            ShortestPath {nodes: vec![], total_cost: -1}
        }
    }
}

#[derive(Debug)]
pub struct ShortestPath {
    nodes: Vec<ShortestPathNode>,
    total_cost: i32,
}

#[derive(Debug)]
pub struct ShortestPathNode {
    name: String,
    piled_cost: i32,
}

impl ShortestPath {
    pub fn get_path_string<'a>(&self, connector: &'a str) -> String {
        let mut stock = vec![];
        for s in &self.nodes {
            stock.push(s.name.clone());
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
    fn it_works() {
        let mut g = new_graph();
        g.add("s", "a", 2);
        g.add("s", "b", 5);
        g.add("a", "b", 2);
        g.add("a", "c", 5);
        g.add("b", "c", 4);
        g.add("b", "d", 2);
        g.add("c", "z", 7);
        g.add("d", "c", 5);
        g.add("d", "z", 2);
        let result = g.shortest_path("s", "z");

        assert_eq!(8, result.cost());
        assert_eq!("s->a->b->d->z", result.get_path_string("->"));
    }
}
