use std::collections::HashMap;
use super::{edge::Edge, node::Node};

pub struct Graph {
    pub nodes: HashMap<uuid::Uuid, Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn neighbors(&self, node_id: uuid::Uuid) -> Vec<&Node> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id)
            .filter_map(|e| self.nodes.get(&e.to))
            .collect()
    }
}
