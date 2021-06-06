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
    let res = NEXT_PROGRAM_FILE_ID.get_or_init(|| ProgramFileId(1)).clone();
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

pub trait Token {
    fn span(&self) -> Span;
}

pub trait TokenSetMatch<Set: ?Sized> {
    fn token_match(set: &Set) -> bool;
}

pub trait TokenSet {
    fn token_match<U: TokenSetMatch<Self>>(&self) -> bool {
        U::token_match(self)
    }
}



pub struct Tokens<T: TokenSet> {
    ts: Vec<T>,
    i: usize,
}

impl<T: TokenSet> Tokens<T> {
    pub fn new(tokens: Vec<T>) -> Tokens<T> {
        Tokens {
            ts: tokens,
            i: 0,
        }
    }
    pub fn get(&self, i: usize) -> &T {
        self.ts.get(i).unwrap()
    }
    pub fn next(&mut self) {
        self.i += 1;
    }
    pub fn parse<P: Parse<T>>(&mut self) -> ParserResult<P> {
        P::parse(self)
    }
}

/*
pub trait Lexer {
    type Item: Token;
    fn lex(s: &str) -> Tokens<Self::Item>;
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxTreeId(usize);

const NEXT_SYNTAX_TREE_ID: OnceCell<SyntaxTreeId> = OnceCell::new();
pub fn gen_next_syntax_tree_id() -> SyntaxTreeId {
    let res = NEXT_SYNTAX_TREE_ID.get_or_init(|| SyntaxTreeId(1)).clone();
    NEXT_SYNTAX_TREE_ID.set(SyntaxTreeId(res.0 + 1)).unwrap();
    res.clone()
}

pub trait SyntaxTree {
    fn id(&self) -> SyntaxTreeId;
}

pub enum ParserResult<T> {
    Ok(T),
    Fail,
    Err,
}

pub trait Parse<T: TokenSet> where Self: Sized {
    fn parse(tokens: &mut Tokens<T>) -> ParserResult<Self>;
}
