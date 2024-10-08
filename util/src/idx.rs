use std::{
  fmt::Debug,
  hash::Hash,
  marker::PhantomData,
  ops::{Index, IndexMut},
};

#[doc(hidden)]
pub use nohash_hasher::IsEnabled;

pub use nohash_hasher::{IntMap, IntSet};

pub trait Idx: Copy + Eq + Ord + Hash + IsEnabled + From<usize> + Into<usize> + Debug {}
impl Idx for usize {}

#[macro_export]
macro_rules! new_idx {
  ($vis:vis $Ty:ident) => {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    $vis struct $Ty(pub usize);

    impl Into<usize> for $Ty {
      fn into(self) -> usize { self.0 }
    }

    impl From<usize> for $Ty {
      fn from(i: usize) -> Self { Self(i) }
    }

    impl $crate::idx::IsEnabled for $Ty {}
    impl $crate::idx::Idx for $Ty {}
  };
}

#[allow(non_snake_case)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexVec<I: Idx, T> {
  vec: Vec<T>,
  PhantomData: PhantomData<fn(&I)>,
}

impl<I: Idx, T> IndexVec<I, T> {
  #[inline(always)]
  pub const fn new() -> Self {
    IndexVec { vec: Vec::new(), PhantomData }
  }

  #[inline(always)]
  pub fn len(&self) -> usize {
    self.vec.len()
  }

  #[inline(always)]
  pub fn next_index(&self) -> I {
    self.len().into()
  }

  #[inline(always)]
  pub fn push(&mut self, value: T) -> I {
    let index = self.next_index();
    self.vec.push(value);
    index
  }

  #[inline(always)]
  pub fn get(&self, index: I) -> Option<&T> {
    self.vec.get(index.into())
  }

  #[inline(always)]
  pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
    self.vec.get_mut(index.into())
  }

  #[inline(always)]
  pub fn into_iter(self) -> impl Iterator<Item = (I, T)> {
    self.vec.into_iter().enumerate().map(map_entry)
  }

  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = (I, &T)> {
    self.vec.iter().enumerate().map(map_entry)
  }

  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (I, &mut T)> {
    self.vec.iter_mut().enumerate().map(map_entry)
  }

  #[inline(always)]
  pub fn keys(&self) -> impl Iterator<Item = I> + Clone {
    (0..self.vec.len()).map(I::from)
  }

  #[inline(always)]
  pub fn into_values(self) -> impl Iterator<Item = T> {
    self.vec.into_iter()
  }

  #[inline(always)]
  pub fn values(&self) -> impl Iterator<Item = &T> {
    self.vec.iter()
  }

  #[inline(always)]
  pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.vec.iter_mut()
  }

  pub fn get_or_extend(&mut self, index: I) -> &mut T
  where
    T: Default,
  {
    self.get_or_extend_with(index, T::default)
  }

  pub fn get_or_extend_with(&mut self, index: I, f: impl FnMut() -> T) -> &mut T {
    let index: usize = index.into();
    if index >= self.vec.len() {
      self.vec.resize_with(index + 1, f);
    }
    &mut self.vec[index]
  }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
  type Output = T;

  fn index(&self, index: I) -> &T {
    &self.vec[index.into()]
  }
}

impl<I: Idx, T> IndexMut<I> for IndexVec<I, T> {
  fn index_mut(&mut self, index: I) -> &mut T {
    &mut self.vec[index.into()]
  }
}

impl<I: Idx + Debug, T: Debug> Debug for IndexVec<I, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut f = f.debug_map();
    f.entries(self.iter());
    f.finish()
  }
}

impl<I: Idx, T> From<Vec<T>> for IndexVec<I, T> {
  fn from(vec: Vec<T>) -> Self {
    IndexVec { vec, PhantomData }
  }
}

impl<I: Idx, T> From<IndexVec<I, T>> for Vec<T> {
  fn from(value: IndexVec<I, T>) -> Self {
    value.vec
  }
}

impl<I: Idx, T> Default for IndexVec<I, T> {
  fn default() -> Self {
    Self { vec: Default::default(), PhantomData }
  }
}

fn map_entry<I: Idx, T>((index, value): (usize, T)) -> (I, T) {
  (index.into(), value)
}
