use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::hash_map::{Iter, Keys};
use std::hash::Hash;

pub struct Graph<N, ND, ED> {
    nodes: HashMap<N, ND>,
    edges: HashMap<N, HashMap<N, ED>>,
}

impl<N, ND, ED> Graph<N, ND, ED>
    where N: Copy + Eq + Hash,
          ED: Copy
{
    pub fn new() -> Graph<N, ND, ED> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: N, data: ND) {
        self.nodes.insert(node, data); // Insert node, if it already exists data will be updated.

        if !self.nodes.contains_key(&node) {
            self.edges.insert(node, HashMap::new());
        }
    }

    pub fn node_data(&self, node: N) -> Option<&ND> {
        self.nodes.get(&node)
    }

    pub fn nodes(&self) -> Keys<N, ND> {
        self.nodes.keys()
    }

    pub fn nodes_data(&self) -> Iter<N, ND> {
        self.nodes.iter()
    }

    pub fn add_edge(&mut self, u: N, v: N, data: ED) {
        // if !self.nodes.contains_key(&u) {
        //     // TODO: Cant add edge if node doesnt exist.
        // }
        // if !self.nodes.contains_key(&v) {
        //     // TODO: Cant add edge if node doesnt exist.
        // }

        // Add edges.
        self.edges
            .entry(u)
            .or_insert(HashMap::new())
            .insert(v, data);
        self.edges
            .entry(v)
            .or_insert(HashMap::new())
            .insert(u, data);
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node1", 2);
        graph.add_node("node3", 3);

        assert!(graph.nodes["node1"] == 2);
        assert!(graph.nodes["node3"] == 3);
    }

    #[test]
    fn test_node_data() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node2", 2);

        assert!(*graph.node_data("node1").unwrap() == 1);
        assert!(*graph.node_data("node2").unwrap() == 2);
        assert!(graph.node_data("no_node") == None);
    }

    #[test]
    fn test_nodes() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node2", 2);
        graph.add_node("node3", 3);

        let nodes: Vec<_> = graph.nodes().cloned().collect();

        assert!(nodes.contains(&"node1"));
        assert!(nodes.contains(&"node2"));
        assert!(nodes.contains(&"node3"));
    }

    #[test]
    fn test_nodes_data() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node2", 2);
        graph.add_node("node3", 3);

        let nodes_data: Vec<_> = graph.nodes_data().collect();

        assert!(nodes_data.contains(&(&"node1", &1)));
        assert!(nodes_data.contains(&(&"node2", &2)));
        assert!(nodes_data.contains(&(&"node3", &3)));
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node1", 2);
        graph.add_node("node3", 3);

        graph.add_edge("node1", "node2", 1);
        graph.add_edge("node1", "node2", 2);
        graph.add_edge("node2", "node3", 3);

        assert!(graph.edges["node1"]["node2"] == 2);
        assert!(graph.edges["node2"]["node3"] == 3);
    }
}
