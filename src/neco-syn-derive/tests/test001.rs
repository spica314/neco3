use neco_syn_derive::*;
use neco_syn;
use neco_syn::{Parse, TokenSetMatch, TokenSet, ParserResult, Tokens};

pub struct TokenEq {
    c: char,
}

impl neco_syn::Token for TokenEq {
    fn span(&self) -> neco_syn::Span {
        neco_syn::Span::new()
    }
}

pub struct TokenLit {
    c: char,
}

impl neco_syn::Token for TokenLit {
    fn span(&self) -> neco_syn::Span {
        neco_syn::Span::new()
    }
}

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

#[derive(Parse)]
#[TokenSet(TestToken)]
pub struct Def {
    ident: TokenLit,
    eq: TokenEq,
    expr: TokenLit,
}

#[test]
#[should_panic]
fn main() {
    let mut tokens: Tokens<TestToken> = Tokens::new(vec![]);
    let t = tokens.parse::<Def>();
}
