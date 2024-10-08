use std::fmt::{self, Debug};

use util::{
  graph::{Edge, Graph},
  idx::IndexVec,
  new_idx,
};

use crate::arrow::Arrow;

#[derive(Debug, Default)]
pub struct Diagram {
  pub nodes: IndexVec<NodeId, NodeType>,
  pub graph: Graph<NodeId, Arrow>,
}

new_idx!(pub NodeId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
  Principal,
  Auxiliary,
}

impl Diagram {
  pub fn conjunction(&mut self) -> (NodeId, NodeId, NodeId) {
    let p0 = self.nodes.push(NodeType::Principal);
    let p1 = self.nodes.push(NodeType::Auxiliary);
    let p2 = self.nodes.push(NodeType::Auxiliary);
    self.graph.insert(p0, p1, Arrow(0b00011));
    self.graph.insert(p0, p2, Arrow(0b00011));
    (p0, p1, p2)
  }

  pub fn disjunction(&mut self) -> (NodeId, NodeId, NodeId) {
    let p0 = self.nodes.push(NodeType::Principal);
    let p1 = self.nodes.push(NodeType::Auxiliary);
    let p2 = self.nodes.push(NodeType::Auxiliary);
    self.graph.insert(p0, p1, Arrow(0b00001));
    self.graph.insert(p0, p2, Arrow(0b00001));
    (p0, p1, p2)
  }

  pub fn complete(&self) -> Self {
    Self { nodes: self.nodes.clone(), graph: self.graph.closure(|_, x, _, y, _| Arrow::join(x, y)) }
  }

  pub fn link(&mut self, a: NodeId, b: NodeId) {
    self.graph.insert(
      a,
      b,
      match (self.nodes[a], self.nodes[b]) {
        (NodeType::Principal, NodeType::Principal) => todo!(),
        (NodeType::Principal, NodeType::Auxiliary) => Arrow(0b11000),
        (NodeType::Auxiliary, NodeType::Principal) => Arrow(0b00011),
        (NodeType::Auxiliary, NodeType::Auxiliary) => Arrow(0b00011),
      },
    );
  }

  pub fn is_contradictory(&self) -> bool {
    self.graph.nodes.values().any(|x| x.edges.values().any(|&x| x.0 == 0))
  }

  pub fn is_complete(&self) -> bool {
    for a in self.graph.nodes.keys() {
      for (&b, &ab) in &self.graph.nodes[a].edges {
        for (&c, &bc) in &self.graph.nodes[b].edges {
          if a != c {
            let ac = self.graph.get_edge(a, c).unwrap_or(Arrow(0b11111));
            if ac.merge(ab.join(bc).unwrap_or(Arrow(0b11111))) != ac {
              return false;
            }
          }
        }
      }
    }
    true
  }
}

impl Debug for NodeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
