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

pub trait Token {
    fn span(&self) -> Span;
}

pub trait Lexer {
    type Item;
    fn lex(s: &str) -> Vec<Self::Item>;
}

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
