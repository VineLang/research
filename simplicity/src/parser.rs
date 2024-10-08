use util::{
  lexer::TokenSet,
  parser::{Delimiters, Parser, ParserState},
};

use crate::{
  ast::{Agent, AgentDef, NetDef, Node, RuleDef, System, Var},
  lexer::Token,
  scope::ScopeBuilder,
};

pub struct SimplicityParser<'src> {
  pub state: ParserState<'src, Token>,
  pub agents: ScopeBuilder<'src, Agent, AgentDef>,
  pub vars: ScopeBuilder<'src, Var, ()>,
  pub rules: Vec<RuleDef>,
  pub nets: Vec<NetDef>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ParseError<'src> {
  LexError,
  UnexpectedToken { expected: TokenSet<Token>, found: &'src str },
  DuplicateAgentDef { name: &'src str },
  UndefinedAgent { name: &'src str },
}

type Parse<'src, T = ()> = Result<T, ParseError<'src>>;

impl<'src> Parser<'src> for SimplicityParser<'src> {
  type Token = Token;
  type Error = ParseError<'src>;

  fn state(&mut self) -> &mut ParserState<'src, Self::Token> {
    &mut self.state
  }

  fn lex_error(&self) -> Self::Error {
    ParseError::LexError
  }

  fn unexpected_error(&self) -> ParseError<'src> {
    ParseError::UnexpectedToken { expected: self.state.expected, found: self.state.lexer.slice() }
  }
}

impl<'src> SimplicityParser<'src> {
  pub fn parse(src: &'src str) -> Parse<'src, System> {
    let mut parser = SimplicityParser {
      state: ParserState::new(src),
      agents: ScopeBuilder::default(),
      vars: ScopeBuilder::default(),
      rules: Vec::new(),
      nets: Vec::new(),
    };
    parser.bump()?;
    while parser.state.token.is_some() {
      parser.parse_def()?;
    }
    Ok(System { agents: parser.agents.finish(), rules: parser.rules, nets: parser.nets })
  }

  fn parse_def(&mut self) -> Parse<'src, ()> {
    if self.check(Token::Agent) {
      self.parse_agent_def()?;
    }
    if self.check(Token::Rule) {
      self.parse_rule_def()?;
    }
    if self.check(Token::Net) {
      self.parse_net_def()?;
    }
    Ok(())
  }

  fn parse_partition<T>(
    &mut self,
    outer_delims: Delimiters<Token>,
    mut parse_el: impl FnMut(&mut Self) -> Parse<'src, T>,
  ) -> Parse<'src, Vec<Vec<T>>> {
    self.parse_delimited(outer_delims, |self_| {
      if self_.eat(Token::OpenBrace)? {
        self_.parse_delimited(
          Delimiters { open: None, close: Some(Token::CloseBrace), separator: Some(Token::Comma) },
          &mut parse_el,
        )
      } else {
        Ok(vec![parse_el(self_)?])
      }
    })
  }

  fn parse_agent_def(&mut self) -> Parse<'src, ()> {
    self.expect(Token::Agent)?;
    let name = self.expect(Token::Ident)?;
    self.expect(Token::OpenParen)?;
    self.expect(Token::Star)?;
    let auxiliary = if self.eat(Token::Comma)? {
      self.parse_partition(
        Delimiters { open: None, close: Some(Token::CloseParen), separator: Some(Token::Comma) },
        |self_| {
          self_.expect(Token::Star)?;
          Ok(())
        },
      )?
    } else {
      self.expect(Token::CloseParen)?;
      Vec::new()
    };
    self
      .agents
      .define(name, AgentDef { auxiliary })
      .map_err(|_| ParseError::DuplicateAgentDef { name })?;
    Ok(())
  }

  fn parse_rule_def(&mut self) -> Parse<'src, ()> {
    self.expect(Token::Rule)?;
    let a = self.parse_node()?;
    let b = self.parse_node()?;
    let result = self.parse_net()?;
    self.rules.push(RuleDef { vars: self.vars.finish(), a, b, result });
    Ok(())
  }

  fn parse_net(&mut self) -> Parse<'src, Vec<Node>> {
    self.parse_delimited(
      Delimiters { open: Some(Token::OpenBrace), close: Some(Token::CloseBrace), separator: None },
      Self::parse_node,
    )
  }

  fn parse_node(&mut self) -> Parse<'src, Node> {
    let name = self.expect(Token::Ident)?;
    let agent = self.agents.get(name).ok_or(ParseError::UndefinedAgent { name })?;
    let ports = self.parse_delimited(
      Delimiters {
        open: Some(Token::OpenParen),
        close: Some(Token::CloseParen),
        separator: Some(Token::Comma),
      },
      Self::parse_var,
    )?;
    Ok(Node { agent, ports })
  }

  fn parse_var(&mut self) -> Parse<'src, Var> {
    let name = self.expect(Token::Ident)?;
    Ok(self.vars.get_or_define(name, ()))
  }

  fn parse_net_def(&mut self) -> Parse<'src, ()> {
    self.expect(Token::Net)?;
    let name = self.expect(Token::Ident)?;
    let ports = self.parse_partition(
      Delimiters {
        open: Some(Token::OpenParen),
        close: Some(Token::CloseParen),
        separator: Some(Token::Comma),
      },
      Self::parse_var,
    )?;
    let nodes = self.parse_net()?;
    self.nets.push(NetDef { name: name.to_owned(), vars: self.vars.finish(), ports, nodes });
    Ok(())
  }
}
