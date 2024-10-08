pub mod arrow;
pub mod ast;
pub mod diagram;
pub mod lexer;
pub mod parser;
pub mod scope;

use diagram::Diagram;
use parser::SimplicityParser;

fn main() {
  let src = include_str!("../examples/nat.nets");
  let system = SimplicityParser::parse(src).unwrap();

  let mut diagram = Diagram::default();

  let net = &system.nets[0];

  diagram.insert_free_ports(net.ports.iter().flatten().copied(), net.ports.iter().map(|x| x.len()));

  for node in &net.nodes {
    let agent = &system.agents.defs[node.agent].value;
    diagram.insert_agent(node.ports.iter().copied(), agent.auxiliary.iter().map(|x| x.len()));
  }

  while !diagram.is_complete() {
    diagram = diagram.complete()
  }

  let consistent = !diagram.is_contradictory();
  dbg!(consistent);
}
