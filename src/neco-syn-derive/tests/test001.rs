use neco_syn_derive::*;
use neco_syn;
use neco_syn::{Parse, TokenSetMatch, TokenSet, ParserResult, Tokens};

#[derive(Debug, Clone)]
pub struct TokenEq {
    c: char,
}

impl neco_syn::Token for TokenEq {
    fn span(&self) -> neco_syn::Span {
        neco_syn::Span::new()
    }
}

#[derive(Debug, Clone)]
pub struct TokenLit {
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
#[derive(Parse)]
#[TokenSet(TestToken)]
pub struct Def {
    ident: TokenLit,
    eq: TokenEq,
    expr: TokenLit,
}

#[derive(Debug, Parse)]
#[TokenSet(TestToken)]
pub struct Def2 {
    eq: TokenEq,
}

#[derive(Debug, Parse)]
#[TokenSet(TestToken)]
pub enum DefOrDef2 {
    Def(Def),
    Def2(Def2),
}

#[test]
fn test001_success_1() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ c: 'x' }),
        TestToken::Eq(TokenEq{ c: '=' }),
        TestToken::Lit(TokenLit{ c: '1' }),
    ]);
    let t = tokens.parse::<Def>();
    assert!(t.is_ok());
}

#[test]
fn test001_success_2() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ c: 'x' }),
        TestToken::Eq(TokenEq{ c: '=' }),
        TestToken::Lit(TokenLit{ c: '1' }),
    ]);
    let t = tokens.parse::<DefOrDef2>();
    assert!(t.is_ok());
}

#[test]
fn test001_success_3() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Eq(TokenEq{ c: '=' }),
    ]);
    let t = tokens.parse::<DefOrDef2>();
    assert!(t.is_ok());
}

#[test]
fn test001_fail() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![
        TestToken::Lit(TokenLit{ c: 'x' }),
        TestToken::Eq(TokenEq{ c: '=' }),
        TestToken::Eq(TokenEq{ c: '=' }),
    ]);
    let t = tokens.parse::<Def>();
    assert!(!t.is_ok())
}
