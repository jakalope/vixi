use std::collections::VecDeque;
use std::collections::vec_deque::{Drain, Iter};
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RemapType {
    NotRelavant, // For Op mapping.
    Remap, // Recursively mappable keys.
    Noremap, // Keys that may be modified by mapping only once.
    Script, // Keys that may only be remapped by script-local mappings.
    Abbreviation, // Don't remap, apply abbreviations.
}

#[derive(Debug, PartialEq)]
pub struct Typeahead<K>
where
    K: Ord,
    K: Copy,
{
    buffer: VecDeque<(K, RemapType)>,
}

pub struct TypeaheadValueIterator<'a, K>
where
    K: Ord,
    K: Copy,
    K: 'a,
{
    buffer_iter: Iter<'a, (K, RemapType)>,
}

impl<'a, K> Iterator for TypeaheadValueIterator<'a, K>
where
    K: Ord,
    K: Copy,
{
    type Item = K;

    fn next(&mut self) -> Option<K> {
        match self.buffer_iter.next() {
            Some(val) => Some(val.0),
            None => None,
        }
    }
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

    pub fn iter(&self) -> Iter<(K, RemapType)> {
        self.buffer.iter()
    }

    /// Provides a `Typeahead` iterator over `K`, omitting the `RemapType`.
    pub fn value_iter(&self) -> TypeaheadValueIterator<K> {
        TypeaheadValueIterator { buffer_iter: self.iter() }
    }

    pub fn pop_front(&mut self) -> Option<(K, RemapType)> {
        self.buffer.pop_front()
    }

    pub fn push_front(&mut self, value: K, remap_type: RemapType) {
        self.buffer.push_front((value, remap_type));
    }

    pub fn push_back(&mut self, value: K, remap_type: RemapType) {
        self.buffer.push_back((value, remap_type));
    }

    /// Appends `value` to the front of the `Typeahead` buffer.
    pub fn put_front(&mut self, value: &Vec<K>, remap_type: RemapType) {
        for k in value.iter().rev() {
            self.push_front(*k, remap_type);
        }
    }

    pub fn drain(&mut self, range: Range<usize>) -> Drain<(K, RemapType)> {
        self.buffer.drain(range)
    }
}
