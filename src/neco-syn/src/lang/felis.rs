#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenTree {
    Symbol(char),
    Number(String),
    Spaces,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokens {
    tokens: Vec<(TokenRange, TokenTree)>,
}

impl Tokens {
    pub fn new(s: &str) -> Tokens {
        let mut pos_y = 1;
        let mut pos_x = 1;
        let mut tokens = vec![];
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
                tokens.push((TokenRange::new(begin, end), TokenTree::Spaces));
                continue;
            }
            if cs[i].is_ascii_digit() {
                let begin = (pos_y, pos_x);
                let mut buf = vec![];
                while i < cs.len() {
                    if cs[i].is_ascii_digit() {
                        buf.push(cs[i]);
                        pos_x += 1;
                        i += 1;
                        continue;
                    }
                    break;
                }
                let end = (pos_y, pos_x);
                let s: String = buf.iter().collect();
                tokens.push((TokenRange::new(begin, end), TokenTree::Number(s)));
                continue;
            }
            if cs[i].is_ascii_alphabetic() {
                unimplemented!();
                continue;
            }
            {
                let begin = (pos_y, pos_x);
                let c = cs[i];
                pos_x += 1;
                i += 1;
                let end = (pos_y, pos_x);
                tokens.push((TokenRange::new(begin, end), TokenTree::Symbol(c)));
            }
        }
        Tokens {
            tokens,
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
        let right = Tokens {
            tokens: vec![
                (TokenRange::new((1,1),(1,2)), TokenTree::Number("1".to_string())),
                (TokenRange::new((1,2),(1,3)), TokenTree::Spaces),
                (TokenRange::new((1,3),(1,4)), TokenTree::Symbol('+')),
                (TokenRange::new((1,4),(1,5)), TokenTree::Spaces),
                (TokenRange::new((1,5),(1,6)), TokenTree::Number("2".to_string())),
                (TokenRange::new((1,6),(1,7)), TokenTree::Spaces),
                (TokenRange::new((1,7),(1,8)), TokenTree::Symbol('*')),
                (TokenRange::new((1,8),(1,9)), TokenTree::Spaces),
                (TokenRange::new((1,9),(1,10)), TokenTree::Number("3".to_string())),
            ],
        };
        assert_eq!(tokens, right);
    }
}
