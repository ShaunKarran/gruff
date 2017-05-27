use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<N, ND, ED> {
    nodes: HashMap<N, ND>,
    edges: HashMap<N, HashMap<N, ED>>,
}

impl<N, ND, ED> Graph<N, ND, ED>
    where N: Copy + Eq + Hash,
          ED: Copy
{
    fn new() -> Graph<N, ND, ED> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: N, data: ND) {
        if self.nodes.contains_key(&node) {
            self.nodes.insert(node, data); // If the key already exists, data will be updated.
        } else {
            self.nodes.insert(node, data);
            self.edges.insert(node, HashMap::new());
        }
    }

    fn add_edge(&mut self, u: N, v: N, data: ED) {
        // Add nodes.
        // if !self.nodes.contains_key(&u) {
        //     let u_clone = u.clone();
        //     self.nodes.insert(u, None);
        //     self.edges.insert(u_clone, HashMap::new());
        // }
        // if !self.nodes.contains_key(&v) {
        //     let v_clone = v.clone();
        //     self.nodes.insert(v, None);
        //     self.edges.insert(v_clone, HashMap::new());
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
        graph.add_node("node2", 2);
        graph.add_node("node1", 3);

        assert!(graph.nodes["node1"] == 3);
        assert!(graph.nodes["node2"] == 2);
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<&str, u32, u32> = Graph::new();

        graph.add_node("node1", 1);
        graph.add_node("node2", 2);
        graph.add_node("node3", 3);

        graph.add_edge("node1", "node2", 10);
        graph.add_edge("node2", "node3", 20);

        assert!(graph.edges["node1"]["node2"] == 10);
        assert!(graph.edges["node2"]["node3"] == 20);
    }
}
