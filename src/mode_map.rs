/// By convention:
/// * K is a map's key type, implementing `Copy` and `Ord`.
/// * T is an arbitrary type, typically stored as a value in a map.
/// * Op is an arbitrary operation type (typically a mode-specific enum).

use disambiguation_map::{DisambiguationMap, Match};
use ordered_vec_map::InsertionResult;
use std::cmp::min;
use typeahead::{Typeahead, RemapType};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum MapErr {
    NoMatch, // No matching op mapping was found.
    InfiniteRecursion, // An infinite loop due to remapping is suspected.
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
    pub fn process(&self, typeahead: &mut Typeahead<K>) -> Result<Op, MapErr> {
        // Grab keys from the front of the queue, looking for matches.
        let mut i: i32 = 0;
        const MAX_REMAP_ITERATIONS: i32 = 1000;
        while typeahead.len() > 0 {
            if i > MAX_REMAP_ITERATIONS {
                return Err(MapErr::InfiniteRecursion);
            }
            i += 1;

            let remap_result =
                self.remap_map.process(typeahead, RemapType::Remap);
            let op_result =
                self.op_map.process(typeahead, RemapType::NotRelavant);

            match (remap_result, op_result) {
                (Match::PartialMatch, _) |
                (_, Match::PartialMatch) => {
                    break; // Ambiguous results.
                }
                (Match::FullMatch(mapped), _) => {
                    // Remapping takes precedence over op-mapping.
                    let len = min(mapped.0.len(), typeahead.len());
                    typeahead.drain(Range { start: 0, end: len });
                    typeahead.put_front(&mapped.1, RemapType::Remap);
                }
                (Match::NoMatch, Match::FullMatch(mapped)) => {
                    // If no remapping, try op-mapping.
                    let len = min(mapped.0.len(), typeahead.len());
                    typeahead.drain(Range { start: 0, end: len });
                    return Ok(mapped.1);
                }
                (Match::NoMatch, Match::NoMatch) => {
                    break; // No matches found.
                }
            }
        }
        return Err(MapErr::NoMatch);
    }

    /// Insert a mapping from `key` to `value` in the operations map.
    /// Empty `key`s are not allowed.
    pub fn insert_op(&mut self, key: Vec<K>, value: Op) -> InsertionResult {
        if key.is_empty() {
            InsertionResult::InvalidKey
        } else {
            self.op_map.insert((key, value))
        }
    }

    /// Insert a mapping from `key` to `value` in the remap map.
    /// Empty `key`s are not allowed and `key` must not equal `value`.
    pub fn insert_remap(
        &mut self,
        key: Vec<K>,
        value: Vec<K>,
    ) -> InsertionResult {
        if key.is_empty() || key == value {
            InsertionResult::InvalidKey
        } else {
            self.remap_map.insert((key, value))
        }
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
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            InsertionResult::Overwrite,
            mode_map.insert_op(vec![1u8], TestOp::ThingTwo)
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Overwrite,
            mode_map.insert_remap(vec![1u8], vec![3u8])
        );
    }

    #[test]
    fn insert_empty_key() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::InvalidKey,
            mode_map.insert_op(Vec::<u8>::new(), TestOp::ThingOne)
        );
        assert_eq!(
            InsertionResult::InvalidKey,
            mode_map.insert_remap(Vec::<u8>::new(), vec![1u8])
        );
    }

    #[test]
    fn insert_key_value_parity() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::InvalidKey,
            mode_map.insert_remap(vec![1u8], vec![1u8])
        );
    }

    #[test]
    fn process_one_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Ok(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_two_ops() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(2u8, RemapType::Remap);

        assert_eq!(Ok(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_overspecified_full_match() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);
        assert_eq!(Ok(TestOp::ThingOne), mode_map.process(&mut typeahead));
    }

    #[test]
    fn process_put_back_leftovers() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Ok(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(Some((2u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_overspecified_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8, 1u8], vec![2u8])
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_shadow_op_with_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Ok(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap_then_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        assert_eq!(Ok(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_op_remap_ambiguate() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8, 1u8, 1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);

        // In this test, we expect the remap not to take place since we haven't
        // yet disambiguated it from the op.
        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(Some((1u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_ambiguous_sequence() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();

        // Ambiguous sequence.
        typeahead.push_back(1u8, RemapType::Remap);
        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(1, typeahead.len());
    }

    #[test]
    fn process_disambiguated_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();

        // Disambiguate for remap.
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(1u8, RemapType::Remap);
        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(Some((2u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(0, typeahead.len());
    }

    #[test]
    fn process_disambiguated_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = Typeahead::<u8>::new();

        // Disambiguated sequence for op.
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(2u8, RemapType::Remap);
        assert_eq!(Ok(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some((2u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_push_front_reversed() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8, 2u8], vec![3u8, 4u8])
        );

        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        typeahead.push_back(2u8, RemapType::Remap);
        assert_eq!(Err(MapErr::NoMatch), mode_map.process(&mut typeahead));
        assert_eq!(Some((3u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(Some((4u8, RemapType::Remap)), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_inf_recursion() {
        // Test that infinite recursion eventually errors out.
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            InsertionResult::Create,
            mode_map.insert_remap(vec![2u8], vec![1u8])
        );
        let mut typeahead = Typeahead::<u8>::new();
        typeahead.push_back(1u8, RemapType::Remap);
        assert_eq!(
            Err(MapErr::InfiniteRecursion),
            mode_map.process(&mut typeahead)
        );
    }
}
