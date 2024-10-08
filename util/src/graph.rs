use nohash_hasher::IntMap;
use std::fmt::Debug;

use crate::{
  bicycle::BicycleState,
  idx::{Idx, IndexVec},
};

mod closure;

#[derive(Debug, Clone)]
pub struct Graph<I: Idx, E: Edge> {
  pub nodes: IndexVec<I, Node<I, E>>,
}

#[derive(Debug)]
pub struct Node<I: Idx, E: Edge> {
  pub edges: IntMap<I, E>,
  state: BicycleState,
}

pub trait Edge: Copy + Debug {
  fn converse(self) -> Option<Self>;
  fn merge(self, other: Self) -> Self;
}

impl<I: Idx, E: Edge> Graph<I, E> {
  pub fn insert(&mut self, a: I, b: I, edge: E) {
    self.half_insert(a, b, edge);
    if let Some(edge) = edge.converse() {
      self.half_insert(b, a, edge);
    } else {
      self.nodes.get_or_extend(b);
    }
  }

  pub fn get_edge(&self, a: I, b: I) -> Option<E> {
    Some(*self.nodes.get(a)?.edges.get(&b)?)
  }

  fn half_insert(&mut self, a: I, b: I, edge: E) {
    self.nodes.get_or_extend(a).edges.entry(b).and_modify(|x| *x = x.merge(edge)).or_insert(edge);
  }

  pub fn bicycle_state(&self, node: I) -> &BicycleState {
    &self.nodes[node].state
  }
}

impl<I: Idx, E: Edge> Default for Graph<I, E> {
  fn default() -> Self {
    Self { nodes: Default::default() }
  }
}

impl<I: Idx, E: Edge> Default for Node<I, E> {
  fn default() -> Self {
    Self { edges: Default::default(), state: Default::default() }
  }
}

impl<I: Idx, E: Edge> Clone for Node<I, E> {
  fn clone(&self) -> Self {
    Self { edges: self.edges.clone(), state: BicycleState::default() }
  }
}
