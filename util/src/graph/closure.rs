use std::mem::take;

use crate::{
  bicycle::{Bicycle, BicycleState},
  idx::Idx,
};

use super::{Edge, Graph};

impl<I: Idx, E: Edge> Graph<I, E> {
  pub fn closure(&self, f: impl Fn(I, E, I, E, I) -> Option<E>) -> Graph<I, E> {
    let mut closure = Closure { input: self, output: self.clone(), f };
    closure.visit_all(self.nodes.keys());
    closure.output
  }
}

struct Closure<'a, I: Idx, E: Edge, F> {
  input: &'a Graph<I, E>,
  output: Graph<I, E>,
  f: F,
}

impl<I: Idx, E: Edge, F: Fn(I, E, I, E, I) -> Option<E>> Bicycle for Closure<'_, I, E, F> {
  type Node = I;

  fn state(&mut self, node: Self::Node) -> &BicycleState {
    self.output.bicycle_state(node)
  }

  fn visit(&mut self, a: Self::Node, mut recurse: impl FnMut(&mut Self, Self::Node)) {
    let node = &self.input.nodes[a];

    for &b in node.edges.keys() {
      recurse(self, b);
    }

    for (&b, &ab) in node.edges.iter() {
      let b_edges = take(&mut self.output.nodes[b].edges);
      for (&c, &bc) in &b_edges {
        if a == c {
          continue;
        }
        if let Some(ac) = (self.f)(a, ab, b, bc, c) {
          self.output.insert(a, c, ac);
        }
      }
      self.output.nodes[b].edges = b_edges;
    }
  }
}
