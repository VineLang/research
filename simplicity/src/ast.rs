use std::fmt::{self, Debug};

use util::new_idx;

use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct System {
  pub agents: Scope<Agent, AgentDef>,
  pub rules: Vec<RuleDef>,
  pub nets: Vec<NetDef>,
}

#[derive(Debug, Clone)]
pub struct AgentDef {
  pub auxiliary: Vec<Vec<()>>,
}

#[derive(Debug, Clone)]
pub struct RuleDef {
  pub vars: Scope<Var, ()>,
  pub a: Node,
  pub b: Node,
  pub result: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct NetDef {
  pub name: String,
  pub vars: Scope<Var, ()>,
  pub ports: Vec<Vec<Var>>,
  pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
  pub agent: Agent,
  pub ports: Vec<Var>,
}

new_idx!(pub Agent);
new_idx!(pub Var);

impl Debug for Agent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "A{}", self.0)
  }
}

impl Debug for Var {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "v{}", self.0)
  }
}
