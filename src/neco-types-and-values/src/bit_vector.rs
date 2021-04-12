use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitVector {
    deque: VecDeque<bool>,
}

impl BitVector {
    pub fn new() -> BitVector {
        BitVector {
            deque: VecDeque::new(),
        }
    }
    pub fn push_lsb(&mut self, v: bool) {
        self.deque.push_front(v);
    }
    pub fn push_msb(&mut self, v: bool) {
        self.deque.push_back(v);
    }
    pub fn pop_lsb(&mut self) -> Option<bool> {
        self.deque.pop_front()
    }
    pub fn pop_msb(&mut self) -> Option<bool> {
        self.deque.pop_back()
    }
    pub fn lsb(&self) -> Option<&bool> {
        self.deque.front()
    }
    pub fn msb(&self) -> Option<&bool> {
        self.deque.back()
    }
    pub fn len(&self) -> usize {
        self.len()
    }
}
