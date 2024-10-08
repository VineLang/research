mod arrow;
mod diagram;

use diagram::Diagram;

fn main() {
  let mut diagram = Diagram::default();

  let r = diagram.conjunction();
  let a = diagram.conjunction();
  let b = diagram.disjunction();

  diagram.link(r.1, a.0);
  diagram.link(r.2, b.0);
  diagram.link(a.1, b.1);
  diagram.link(a.2, b.2);

  let completion = diagram.complete();
  dbg!(&completion);
  assert!(completion.is_complete());

  let consistent = !completion.is_contradictory();
  dbg!(consistent);
}
