use std::{collections::HashMap, mem::take};

use util::idx::{Idx, IndexVec};

#[derive(Debug, Clone)]
pub struct Scope<K: Idx, T> {
  pub defs: IndexVec<K, Definition<T>>,
}

#[derive(Debug, Clone)]
pub struct Definition<T> {
  pub name: String,
  pub value: T,
}

impl<I: Idx, T> Scope<I, T> {
  pub fn define(&mut self, name: String, value: T) -> I {
    self.defs.push(Definition { name, value })
  }
}

impl<K: Idx, T> Default for Scope<K, T> {
  fn default() -> Self {
    Self { defs: Default::default() }
  }
}

pub struct ScopeBuilder<'src, I: Idx, T> {
  pub scope: Scope<I, T>,
  pub lookup: HashMap<&'src str, I>,
}

impl<'src, I: Idx, T> ScopeBuilder<'src, I, T> {
  pub fn define(&mut self, name: &'src str, value: T) -> Result<I, ()> {
    let index = self.scope.defs.next_index();
    if self.lookup.insert(name, index).is_some() {
      return Err(());
    }
    self.scope.define(name.to_owned(), value);
    Ok(index)
  }

  pub fn get(&mut self, name: &'src str) -> Option<I> {
    self.lookup.get(name).copied()
  }

  pub fn get_or_define(&mut self, name: &'src str, value: T) -> I {
    *self.lookup.entry(name).or_insert_with(|| self.scope.define(name.to_owned(), value))
  }

  pub fn finish(&mut self) -> Scope<I, T> {
    self.lookup.clear();
    take(&mut self.scope)
  }
}

impl<'src, I: Idx, T> Default for ScopeBuilder<'src, I, T> {
  fn default() -> Self {
    Self { scope: Default::default(), lookup: Default::default() }
  }
}
