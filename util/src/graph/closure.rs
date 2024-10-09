use std::{
  collections::{hash_map::Entry, HashSet},
  mem::take,
};

use crate::idx::Idx;

use super::{Edge, Graph};

impl<I: Idx, E: Edge> Graph<I, E> {
  pub fn closure(&mut self, f: impl Fn(I, E, I, E, I) -> Option<E>) {
    let todo = self
      .nodes
      .iter()
      .flat_map(|(a, n)| n.edges.keys().filter(move |&&b| b > a).map(move |&b| (a, b)))
      .collect();
    let mut closure = Closure { graph: self, todo, f };
    while let Some(&(a, b)) = closure.todo.iter().next() {
      closure.todo.remove(&(a, b));
      closure.process_edge(a, b);
    }
  }
}

struct Closure<'a, I: Idx, E: Edge, F> {
  graph: &'a mut Graph<I, E>,
  todo: HashSet<(I, I)>,
  f: F,
}

impl<'a, I: Idx, E: Edge, F: Fn(I, E, I, E, I) -> Option<E>> Closure<'a, I, E, F> {
  fn process_edge(&mut self, a: I, b: I) {
    self.half_process_edge(a, b);
    self.half_process_edge(b, a);
  }

  fn half_process_edge(&mut self, a: I, b: I) {
    let Some(ab) = self.graph.get_edge(a, b) else { return };

    let edges = take(&mut self.graph.nodes[b].edges);
    for (&c, &bc) in &edges {
      let Some(mut ac) = (self.f)(a, ab, b, bc, c) else { continue };
      match self.graph.nodes[a].edges.entry(c) {
        Entry::Occupied(mut e) => {
          let e = e.get_mut();
          ac = e.merge(ac);
          if *e == ac {
            continue;
          }
          *e = ac;
        }
        Entry::Vacant(e) => {
          e.insert(ac);
        }
      }
      if let Some(ca) = ac.converse() {
        self.graph.half_insert(c, a, ca);
      }
      self.todo.insert(if a < c { (a, c) } else { (c, a) });
    }
    self.graph.nodes[b].edges = edges;
  }
}
