use std::{
  collections::hash_map::Entry,
  fmt::{self, Debug},
};

use util::{
  graph::{Edge, Graph},
  idx::{IndexVec, IntMap},
  new_idx,
};

use crate::{arrow::Arrow, ast::Var};

#[derive(Debug, Default)]
pub struct Diagram {
  pub vars: IntMap<Var, NodeId>,
  pub nodes: IndexVec<NodeId, NodeType>,
  pub graph: Graph<NodeId, Arrow>,
}

new_idx!(pub NodeId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
  Principal,
  Auxiliary,
  Partition,
}

impl Diagram {
  pub fn insert_free_ports(
    &mut self,
    vars: impl IntoIterator<Item = Var>,
    partition: impl IntoIterator<Item = usize>,
  ) {
    let mut vars = vars.into_iter();
    for n in partition {
      let x = self.nodes.push(NodeType::Principal);
      for v in (&mut vars).take(n) {
        self.link_var(v, x);
      }
    }
  }

  pub fn insert_agent(
    &mut self,
    vars: impl IntoIterator<Item = Var>,
    partition: impl IntoIterator<Item = usize>,
  ) {
    let principal = self.nodes.push(NodeType::Principal);
    let mut vars = vars.into_iter();
    self.link_var(vars.next().unwrap(), principal);
    for n in partition {
      let partition = self.nodes.push(NodeType::Partition);
      self.graph.insert(principal, partition, Arrow(0b00001));
      for v in (&mut vars).take(n) {
        let aux = self.nodes.push(NodeType::Auxiliary);
        self.graph.insert(partition, aux, Arrow(0b00011));
        self.link_var(v, aux);
      }
    }
  }

  fn link_var(&mut self, var: Var, node: NodeId) {
    match self.vars.entry(var) {
      Entry::Occupied(e) => {
        let other = e.remove();
        self.link(node, other);
      }
      Entry::Vacant(e) => {
        e.insert(node);
      }
    }
  }

  pub fn complete(&self) -> Self {
    Self {
      vars: Default::default(),
      nodes: self.nodes.clone(),
      graph: self.graph.closure(|_, x, _, y, _| Arrow::join(x, y)),
    }
  }

  pub fn link(&mut self, a: NodeId, b: NodeId) {
    self.graph.insert(
      a,
      b,
      match (self.nodes[a], self.nodes[b]) {
        (NodeType::Principal, NodeType::Principal) => Arrow(0b00100),
        (NodeType::Principal, NodeType::Auxiliary) => Arrow(0b11000),
        (NodeType::Auxiliary, NodeType::Principal) => Arrow(0b00011),
        (NodeType::Auxiliary, NodeType::Auxiliary) => Arrow(0b00011),
        (NodeType::Partition, _) | (_, NodeType::Partition) => unreachable!(),
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
