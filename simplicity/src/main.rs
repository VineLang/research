pub mod arrow;
pub mod ast;
pub mod diagram;
pub mod lexer;
pub mod parser;
pub mod scope;

use std::{env::args, fs::read_to_string};

use diagram::Diagram;
use parser::SimplicityParser;

fn main() {
  let path = args().nth(1).expect("must supply path");
  let src = read_to_string(path).unwrap();
  let system = SimplicityParser::parse(&src).unwrap();

  for rule in &system.rules {
    let mut diagram = Diagram::default();

    let a = &system.agents.defs[rule.a.agent];
    let b = &system.agents.defs[rule.b.agent];
    diagram.insert_free_ports(
      rule.a.ports[1..].iter().copied(),
      a.value.auxiliary.iter().map(|x| x.len()),
    );
    diagram.insert_free_ports(
      rule.b.ports[1..].iter().copied(),
      b.value.auxiliary.iter().map(|x| x.len()),
    );

    for node in &rule.result {
      let agent = &system.agents.defs[node.agent].value;
      diagram.insert_agent(node.ports.iter().copied(), agent.auxiliary.iter().map(|x| x.len()));
    }

    diagram.complete();
    assert!(diagram.is_complete());

    let simple = !diagram.is_contradictory();
    println!("rule {}/{}: {}", a.name, b.name, if simple { "simple" } else { "non-simple" });
  }

  for net in &system.nets {
    let mut diagram = Diagram::default();

    diagram
      .insert_free_ports(net.ports.iter().flatten().copied(), net.ports.iter().map(|x| x.len()));

    for node in &net.nodes {
      let agent = &system.agents.defs[node.agent].value;
      diagram.insert_agent(node.ports.iter().copied(), agent.auxiliary.iter().map(|x| x.len()));
    }

    diagram.complete();
    assert!(diagram.is_complete());

    let simple = !diagram.is_contradictory();
    println!("net {}: {}", net.name, if simple { "simple" } else { "non-simple" });
  }
}
