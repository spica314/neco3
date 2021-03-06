#![feature(once_cell)]

pub mod lang;

use std::{lazy::OnceCell, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    files: Vec<ProgramFile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramFileId(usize);

const NEXT_PROGRAM_FILE_ID: OnceCell<ProgramFileId> = OnceCell::new();
pub fn gen_next_program_file_id() -> ProgramFileId {
    let res = NEXT_PROGRAM_FILE_ID
        .get_or_init(|| ProgramFileId(1))
        .clone();
    NEXT_PROGRAM_FILE_ID.set(ProgramFileId(res.0 + 1)).unwrap();
    res.clone()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramFile {
    program_file_id: ProgramFileId,
    path: PathBuf,
    body: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    program_file_id: ProgramFileId,
    begin: usize,
    end: usize,
}

impl Span {
    pub fn new() -> Span {
        Span {
            program_file_id: ProgramFileId(0),
            begin: 0,
            end: 0,
        }
    }
    pub fn new_with_span(program_file_id: ProgramFileId, begin: usize, end: usize) -> Span {
        Span {
            program_file_id,
            begin,
            end,
        }
    }
}

pub trait Token: Clone {
    fn span(&self) -> Span;
}

pub trait TokenSetMatch<Set: ?Sized>: Sized {
    fn token_match(set: &Set) -> Option<Self>;
}

pub trait TokenSet {
    fn token_match<U: TokenSetMatch<Self>>(&self) -> Option<U> {
        U::token_match(self)
    }
}

pub struct Tokens<T: TokenSet> {
    ts: Vec<T>,
    i: usize,
}

impl<T: TokenSet> Tokens<T> {
    pub fn new(tokens: Vec<T>) -> Tokens<T> {
        Tokens { ts: tokens, i: 0 }
    }
    pub fn get_i(&self) -> usize {
        self.i
    }
    pub fn set_i(&mut self, i: usize) {
        self.i = i;
    }
    pub fn get_token(&self) -> &T {
        self.ts.get(self.i).unwrap()
    }
    pub fn next(&mut self) {
        self.i += 1;
    }
    pub fn parse<P: SyntaxTree<T>>(&mut self) -> ParserResult<P> {
        P::parse(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxTreeId(usize);

const NEXT_SYNTAX_TREE_ID: OnceCell<SyntaxTreeId> = OnceCell::new();
pub fn gen_next_syntax_tree_id() -> SyntaxTreeId {
    let res = NEXT_SYNTAX_TREE_ID.get_or_init(|| SyntaxTreeId(1)).clone();
    NEXT_SYNTAX_TREE_ID.set(SyntaxTreeId(res.0 + 1)).unwrap();
    res.clone()
}

#[derive(Debug, Clone)]
pub enum ParserResult<T> {
    Ok(T),
    Fail,
    Err,
}

impl<T> ParserResult<T> {
    pub fn is_ok(&self) -> bool {
        matches!(self, ParserResult::Ok(_))
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, ParserResult::Fail)
    }
    pub fn is_err(&self) -> bool {
        matches!(self, ParserResult::Err)
    }
}

pub struct Rep0<T> {
    pub id: SyntaxTreeId,
    pub ts: Vec<T>,
}

pub struct Rep1<T> {
    pub id: SyntaxTreeId,
    pub ts: Vec<T>,
}

pub struct Optional<T> {
    pub id: SyntaxTreeId,
    pub inner: Option<T>,
}

pub trait SyntaxTree<T: TokenSet>
where
    Self: Sized,
{
    fn parse(tokens: &mut Tokens<T>) -> ParserResult<Self>;
    fn id(&self) -> SyntaxTreeId;
}

impl<S: TokenSet, T: SyntaxTree<S>> SyntaxTree<S> for Rep0<T> {
    fn parse(tokens: &mut Tokens<S>) -> ParserResult<Self> {
        let mut res = vec![];
        while let ParserResult::Ok(t) = tokens.parse::<T>() {
            res.push(t);
        }
        ParserResult::Ok(Rep0 {
            id: gen_next_syntax_tree_id(),
            ts: res,
        })
    }
    fn id(&self) -> SyntaxTreeId {
        self.id
    }
}

impl<S: TokenSet, T: SyntaxTree<S>> SyntaxTree<S> for Rep1<T> {
    fn parse(tokens: &mut Tokens<S>) -> ParserResult<Self> {
        let mut res = vec![];
        while let ParserResult::Ok(t) = tokens.parse::<T>() {
            res.push(t);
        }
        if res.len() >= 1 {
            ParserResult::Ok(Rep1 {
                id: gen_next_syntax_tree_id(),
                ts: res,
            })
        } else {
            ParserResult::Fail
        }
    }
    fn id(&self) -> SyntaxTreeId {
        self.id
    }
}

impl<S: TokenSet, T: SyntaxTree<S>> SyntaxTree<S> for Optional<T> {
    fn parse(tokens: &mut Tokens<S>) -> ParserResult<Self> {
        if let ParserResult::Ok(t) = tokens.parse::<T>() {
            ParserResult::Ok(Optional {
                id: gen_next_syntax_tree_id(),
                inner: Some(t),
            })
        } else {
            ParserResult::Ok(Optional {
                id: gen_next_syntax_tree_id(),
                inner: None,
            })
        }
    }
    fn id(&self) -> SyntaxTreeId {
        self.id
    }
}

// accept: (empty), T, T P, T P T, T P T P, T P T P T, ...
pub struct Punctuated<T, P> {
    pub id: SyntaxTreeId,
    pub ts: Vec<T>,
    pub ps: Vec<P>,
}

impl<S: TokenSet, T: SyntaxTree<S>, P: SyntaxTree<S>> SyntaxTree<S> for Punctuated<T, P> {
    fn parse(tokens: &mut Tokens<S>) -> ParserResult<Self> {
        let mut ts = vec![];
        let mut ps = vec![];
        loop {
            if let ParserResult::Ok(t) = tokens.parse::<T>() {
                ts.push(t);
            } else {
                break;
            }
            if let ParserResult::Ok(p) = tokens.parse::<P>() {
                ps.push(p);
            } else {
                break;
            }
        }
        ParserResult::Ok(Punctuated {
            id: gen_next_syntax_tree_id(),
            ts,
            ps,
        })
    }
    fn id(&self) -> SyntaxTreeId {
        self.id
    }
}
