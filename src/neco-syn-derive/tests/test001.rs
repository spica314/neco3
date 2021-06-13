use neco_syn::{Span, SyntaxTreeId};
use neco_syn_derive::*;

#[derive(Debug, Clone, Token)]
pub struct TokenEq {
    id: SyntaxTreeId,
    span: Span,
    c: char,
}

#[derive(Debug, Clone, Token)]
pub struct TokenLit {
    id: SyntaxTreeId,
    span: Span,
    c: char,
}

#[derive(Clone, TokenSet)]
pub enum TestToken {
    Eq(TokenEq),
    Lit(TokenLit),
}

#[derive(Debug, SyntaxTree)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use neco_syn::{gen_next_syntax_tree_id, Tokens};

    #[test]
    fn test001_success_1() {
        let mut tokens: Tokens<TestToken> = Tokens::new(vec![
            TestToken::Lit(TokenLit {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: 'x',
            }),
            TestToken::Eq(TokenEq {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '=',
            }),
            TestToken::Lit(TokenLit {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '1',
            }),
        ]);
        let t = tokens.parse::<Def>();
        assert!(t.is_ok());
    }

    #[test]
    fn test001_success_2() {
        let mut tokens: Tokens<TestToken> = Tokens::new(vec![
            TestToken::Lit(TokenLit {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: 'x',
            }),
            TestToken::Eq(TokenEq {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '=',
            }),
            TestToken::Lit(TokenLit {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '1',
            }),
        ]);
        let t = tokens.parse::<DefOrDef2>();
        assert!(t.is_ok());
    }

    #[test]
    fn test001_success_3() {
        let mut tokens: Tokens<TestToken> = Tokens::new(vec![TestToken::Eq(TokenEq {
            id: gen_next_syntax_tree_id(),
            span: Span::new(),
            c: '=',
        })]);
        let t = tokens.parse::<DefOrDef2>();
        assert!(t.is_ok());
    }

    #[test]
    fn test001_fail() {
        let mut tokens: Tokens<TestToken> = Tokens::new(vec![
            TestToken::Lit(TokenLit {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: 'x',
            }),
            TestToken::Eq(TokenEq {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '=',
            }),
            TestToken::Eq(TokenEq {
                id: gen_next_syntax_tree_id(),
                span: Span::new(),
                c: '=',
            }),
        ]);
        let t = tokens.parse::<Def>();
        assert!(!t.is_ok())
    }
}
