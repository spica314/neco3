use neco_table::{Id, MainTable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenNode {
    Symbol(TokenSymbol),
    Ident(TokenIdent),
    Number(TokenNumber),
    Spaces(TokenSpaces),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenSymbol {
    pub range: TokenRange,
    pub c: char,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenIdent {
    pub range: TokenRange,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenNumber {
    pub range: TokenRange,
    pub number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenSpaces {
    pub range: TokenRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenRange {
    begin: (usize,usize),
    end: (usize,usize),
}

impl TokenRange {
    pub fn new(begin: (usize,usize), end: (usize,usize)) -> TokenRange {
        TokenRange {
            begin,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokens {
    token_table: MainTable<TokenNode>,
    root_tokens: Vec<Id<TokenNode>>,
}

impl Tokens {
    pub fn new(s: &str) -> Tokens {
        let mut pos_y = 1;
        let mut pos_x = 1;
        let mut token_table = MainTable::new();
        let mut root_tokens = vec![];
        let cs: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < cs.len() {
            if cs[i].is_whitespace() {
                let begin = (pos_y, pos_x);
                while i < cs.len() {
                    if cs[i] == '\n' {
                        pos_y += 1;
                        pos_x = 1;
                        i += 1;
                        continue;
                    }
                    if cs[i].is_whitespace() {
                        pos_x += 1;
                        i += 1;
                        continue;
                    }
                    break;
                }
                let end = (pos_y, pos_x);
                let node = TokenNode::Spaces(TokenSpaces {
                    range: TokenRange::new(begin, end),
                });
                let id = token_table.insert(node);
                root_tokens.push(id);
                continue;
            }
            if cs[i].is_ascii_digit() {
                let begin = (pos_y, pos_x);
                let mut buf = vec![];
                while i < cs.len() {
                    if cs[i].is_ascii_digit() || cs[i] == '_' {
                        buf.push(cs[i]);
                        pos_x += 1;
                        i += 1;
                        continue;
                    }
                    break;
                }
                let end = (pos_y, pos_x);
                let s: String = buf.iter().collect();
                let node = TokenNode::Number(TokenNumber {
                    range: TokenRange::new(begin, end),
                    number: s,
                });
                let id = token_table.insert(node);
                root_tokens.push(id);
                continue;
            }
            if cs[i].is_ascii_alphabetic() || cs[i] == '_' {
                let begin = (pos_y, pos_x);
                let mut buf = vec![];
                while i < cs.len() {
                    if cs[i].is_ascii_alphanumeric() || cs[i] == '_' {
                        buf.push(cs[i]);
                        pos_x += 1;
                        i += 1;
                        continue;
                    }
                    break;
                }
                let end = (pos_y, pos_x);
                let s: String = buf.iter().collect();
                let node = TokenNode::Ident(TokenIdent {
                    range: TokenRange::new(begin, end),
                    ident: s,
                });
                let id = token_table.insert(node);
                root_tokens.push(id);
                continue;
            }
            {
                let begin = (pos_y, pos_x);
                let c = cs[i];
                pos_x += 1;
                i += 1;
                let end = (pos_y, pos_x);
                let node = TokenNode::Symbol(TokenSymbol {
                    range: TokenRange::new(begin, end),
                    c,
                });
                let id = token_table.insert(node);
                root_tokens.push(id);
            }
        }
        Tokens {
            token_table,
            root_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_new_1() {
        let s = r#"1 + 2 * 3"#;
        let tokens = Tokens::new(s);
        assert!(tokens.root_tokens.len() == 9);
        let left: Vec<_> = tokens.root_tokens.iter().map(|x| tokens.token_table.get(*x).unwrap().clone() ).collect();
        let right = vec![
            TokenNode::Number(TokenNumber{ range: TokenRange::new((1,1),(1,2)), number: "1".to_string() }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,2),(1,3)) }),
            TokenNode::Symbol(TokenSymbol{ range: TokenRange::new((1,3),(1,4)), c: '+' }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,4),(1,5)) }),
            TokenNode::Number(TokenNumber{ range: TokenRange::new((1,5),(1,6)), number: "2".to_string() }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,6),(1,7)) }),
            TokenNode::Symbol(TokenSymbol{ range: TokenRange::new((1,7),(1,8)), c: '*' }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,8),(1,9)) }),
            TokenNode::Number(TokenNumber{ range: TokenRange::new((1,9),(1,10)), number: "3".to_string() }),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn test_tokens_new_2() {
        let s = r#"abc + d_e * _f"#;
        let tokens = Tokens::new(s);
        assert!(tokens.root_tokens.len() == 9);
        let left: Vec<_> = tokens.root_tokens.iter().map(|x| tokens.token_table.get(*x).unwrap().clone() ).collect();
        let right = vec![
            TokenNode::Ident(TokenIdent{ range: TokenRange::new((1,1),(1,4)), ident: "abc".to_string() }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,4),(1,5)) }),
            TokenNode::Symbol(TokenSymbol{ range: TokenRange::new((1,5),(1,6)), c: '+' }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,6),(1,7)) }),
            TokenNode::Ident(TokenIdent{ range: TokenRange::new((1,7),(1,10)), ident: "d_e".to_string() }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,10),(1,11)) }),
            TokenNode::Symbol(TokenSymbol{ range: TokenRange::new((1,11),(1,12)), c: '*' }),
            TokenNode::Spaces(TokenSpaces{ range: TokenRange::new((1,12),(1,13)) }),
            TokenNode::Ident(TokenIdent{ range: TokenRange::new((1,13),(1,15)), ident: "_f".to_string() }),
        ];
        assert_eq!(left, right);
    }
}
