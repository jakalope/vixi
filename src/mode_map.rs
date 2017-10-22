use std::collections::VecDeque;
use ordered_vec_map::InsertionResult;
use disambiguation_map::{DisambiguationMap, Match};
use termion::event::{Key, parse_event};
use op::{NormalOp, InsertOp};
use std::cmp::{min, max};
use std::cmp::Ordering;

/// By convention:
/// * K is a map's key type, implementing `Copy` and `Ord`.
/// * T is an arbitrary type, typically stored as a value in a map.
/// * Op is an arbitrary operation type (typically a mode-specific enum).
#[derive(Debug, PartialEq)]
pub enum OpErr {
    EmptyKey,
    NotEnoughArgs(usize),
}

#[derive(Debug, PartialEq)]
pub enum RemapErr {
    EmptyKey,
    KeyValueEqual,
}

pub struct ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
    remap_map: DisambiguationMap<K, Vec<K>>,
    op_map: DisambiguationMap<K, Op>,
}


impl<K, Op> ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
    pub fn new() -> Self {
        ModeMap {
            remap_map: DisambiguationMap::new(),
            op_map: DisambiguationMap::new(),
        }
    }

    /// Process a typeahead buffer.
    pub fn process(&self, mut typeahead: &mut VecDeque<K>) -> Option<Op> {
        // Grab keys from the front of the queue, looking for matches.
        let mut i: usize = 0;
        const MAX_REMAP_ITERATIONS: usize = 1000;
        while typeahead.len() > 0 {
            if i > MAX_REMAP_ITERATIONS {
                panic!("Infinite loop suspected.");
            }
            i += 1;

            let remap_result = self.remap_map.process(&mut typeahead);
            let op_result = self.op_map.process(&mut typeahead);

            match (remap_result, op_result) {
                (Match::PartialMatch, _) |
                (_, Match::PartialMatch) => {
                    // Ambiguous results.
                    break;
                }
                (Match::FullMatch(mapped), _) => {
                    // Remapping takes precedence over op-mapping.
                    let len = min(mapped.0.len(), typeahead.len());
                    typeahead.drain(..len);
                    for k in mapped.1.iter() {
                        typeahead.push_front(*k);
                    }
                }
                (Match::NoMatch, Match::FullMatch(mapped)) => {
                    // If no remapping, try op-mapping.
                    let len = min(mapped.0.len(), typeahead.len());
                    typeahead.drain(..len);
                    return Some(mapped.1);
                }
                (Match::NoMatch, Match::NoMatch) => {
                    // No matches found.
                    break;
                }
            }
        }
        return None;
    }

    /// Insert a mapping from `key` to `value` in the operations map.
    /// Empty `key`s are not allowed.
    pub fn insert_op(
        &mut self,
        key: Vec<K>,
        value: Op,
    ) -> Result<InsertionResult, OpErr> {
        // Disallow empty keys, as they would full-match against an empty
        // typeahead buffer.
        if key.is_empty() {
            return Err(OpErr::EmptyKey);
        }
        return Ok(self.op_map.insert((key, value)));
    }

    /// Insert a mapping from `key` to `value` in the remap map.
    /// Empty `key`s are not allowed.
    /// Pairs such that `key` and `value` are equal are not allowed.
    pub fn insert_remap(
        &mut self,
        key: Vec<K>,
        value: Vec<K>,
    ) -> Result<InsertionResult, RemapErr> {
        // Disallow empty keys, as they would full-match against an empty
        // typeahead buffer.
        if key.is_empty() {
            return Err(RemapErr::EmptyKey);
        }
        if key.cmp(&value) == Ordering::Equal {
            return Err(RemapErr::KeyValueEqual);
        }
        return Ok(self.remap_map.insert((key, value)));
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestOp {
        ThingOne,
        ThingTwo,
    }

    #[test]
    fn insert_overwrite() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Overwrite),
            mode_map.insert_op(vec![1u8], TestOp::ThingTwo)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Overwrite),
            mode_map.insert_remap(vec![1u8], vec![3u8])
        );
    }

    #[test]
    fn insert_empty_key() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Err(OpErr::EmptyKey),
            mode_map.insert_op(Vec::<u8>::new(), TestOp::ThingOne)
        );
        assert_eq!(
            Err(RemapErr::EmptyKey),
            mode_map.insert_remap(Vec::<u8>::new(), vec![1u8])
        );
    }

    #[test]
    fn insert_key_value_parity() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Err(RemapErr::KeyValueEqual),
            mode_map.insert_remap(vec![1u8], vec![1u8])
        );
    }

    #[test]
    fn process_one_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_two_ops() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(2u8);

        assert_eq!(Some(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_overspecified_full_match() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);
        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
    }

    #[test]
    fn process_put_back_leftovers() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_overspecified_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8, 1u8], vec![2u8])
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_shadow_op_with_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap_then_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_op_remap_ambiguate() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8, 1u8, 1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        // In this test, we expect the remap not to take place since we haven't
        // yet disambiguated it from the op.
        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_ambiguous_sequence() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Ambiguous sequence.
        typeahead.push_back(1u8);
        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(1, typeahead.len());
    }

    #[test]
    fn process_disambiguated_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Disambiguate for remap.
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);
        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(0, typeahead.len());
    }

    #[test]
    fn process_disambiguated_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Disambiguated sequence for op.
        typeahead.push_back(1u8); // Processing just this will result in None.
        typeahead.push_back(2u8); // This one disambiguates, so the op can map.
        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    // Test that infinite recursion eventuates in a panic.
    // TODO Instead of panic, consider returning a special InfRecursion op
    // so we can tell the user.
    #[test]
    #[should_panic]
    fn process_inf_recursion() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![2u8], vec![1u8])
        );
        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        mode_map.process(&mut typeahead);
    }
}
