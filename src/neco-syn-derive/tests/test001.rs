use neco_syn_derive::*;
use neco_syn;
use neco_syn::{SyntaxTree, SyntaxTreeId, TokenSetMatch, TokenSet, ParserResult, Tokens, gen_next_syntax_tree_id};

#[derive(Debug, Clone)]
pub struct TokenEq {
    id: SyntaxTreeId,
    c: char,
}

impl neco_syn::Token for TokenEq {
    fn span(&self) -> neco_syn::Span {
        neco_syn::Span::new()
    }
}

#[derive(Debug, Clone)]
pub struct TokenLit {
    id: SyntaxTreeId,
    c: char,
}

impl neco_syn::Token for TokenLit {
    fn span(&self) -> neco_syn::Span {
        neco_syn::Span::new()
    }
}

#[derive(Clone)]
#[derive(TokenSet)]
pub enum TestToken {
    Eq(TokenEq),
    Lit(TokenLit),
}

impl neco_syn::Token for TestToken {
    fn span(&self) -> neco_syn::Span {
        match self {
            TestToken::Eq(token_eq) => token_eq.span(),
            TestToken::Lit(token_lit) => token_lit.span(),
        }
    }
}

fn lex(s: &str) -> neco_syn::Tokens<TestToken> {
    neco_syn::Tokens::new(vec![])
}

#[derive(Debug)]
#[derive(SyntaxTree)]
#[TokenSet(TestToken)]
pub struct Def {
    id: SyntaxTreeId,
    ident: TokenLit,
    eq: TokenEq,
    expr: TokenLit,
}

#[derive(Debug, SyntaxTree)]
#[TokenSet(TestToken)]
pub struct Def2 {
    id: SyntaxTreeId,
    eq: TokenEq,
}

#[derive(Debug, SyntaxTree)]
#[TokenSet(TestToken)]
pub enum DefOrDef2 {
    Def(Def),
    Def2(Def2),
}

#[test]
fn test001_success_1() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ id: gen_next_syntax_tree_id(), c: 'x' }),
        TestToken::Eq(TokenEq{ id: gen_next_syntax_tree_id(), c: '=' }),
        TestToken::Lit(TokenLit{ id: gen_next_syntax_tree_id(), c: '1' }),
    ]);
    let t = tokens.parse::<Def>();
    assert!(t.is_ok());
}

#[test]
fn test001_success_2() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ id: gen_next_syntax_tree_id(), c: 'x' }),
        TestToken::Eq(TokenEq{ id: gen_next_syntax_tree_id(), c: '=' }),
        TestToken::Lit(TokenLit{ id: gen_next_syntax_tree_id(), c: '1' }),
    ]);
    let t = tokens.parse::<DefOrDef2>();
    assert!(t.is_ok());
}

#[test]
fn test001_success_3() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Eq(TokenEq{ id: gen_next_syntax_tree_id(), c: '=' }),
    ]);
    let t = tokens.parse::<DefOrDef2>();
    assert!(t.is_ok());
}

#[test]
fn test001_fail() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ id: gen_next_syntax_tree_id(), c: 'x' }),
        TestToken::Eq(TokenEq{ id: gen_next_syntax_tree_id(), c: '=' }),
        TestToken::Eq(TokenEq{ id: gen_next_syntax_tree_id(), c: '=' }),
    ]);
    let t = tokens.parse::<Def>();
    assert!(!t.is_ok())
}
