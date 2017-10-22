use std::collections::VecDeque;
use std::collections::vec_deque::{Drain, Iter};
use std::collections::range::RangeArgument;

pub enum RemapType {
    Remap,
    Noremap,
    Script,
    Abbreviation,
}

pub struct Typeahead<K>
where
    K: Ord,
    K: Copy,
{
    buffer: VecDeque<K>,
}

impl<K> Typeahead<K>
where
    K: Ord,
    K: Copy,
{
    pub fn new() -> Self {
        Typeahead { buffer: VecDeque::new() }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn iter(&self) -> Iter<K> {
        self.buffer.iter()
    }

    pub fn pop_front(&mut self) -> Option<K> {
        self.buffer.pop_front()
    }

    pub fn push_front(&mut self, value: K) {
        self.buffer.push_front(value);
    }

    pub fn push_back(&mut self, value: K) {
        self.buffer.push_back(value);
    }

    pub fn put_front(&mut self, value: &Vec<K>) {
        for k in value.iter().rev() {
            self.buffer.push_front(*k);
        }
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<K>
    where
        R: RangeArgument<usize>,
    {
        self.buffer.drain(range)
    }
}
