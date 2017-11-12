use mode::{insert, normal, InsertMode, Mode, Transition};
use mode_map::MapErr;
use op::InsertOp;
use state::State;
use typeahead::Numeric;

impl<K> Transition<K> for InsertMode<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    fn name(&self) -> &'static str {
        "Insert"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match state.insert_mode_map.process(&mut state.typeahead) {
            Err(MapErr::NoMatch) => {
                // In Insert mode, unmatched typeahead gets inserted.
                // TODO send keystrokes to owner.
            } 
            Err(MapErr::InfiniteRecursion) => {
                // TODO Tell the user they've created an infinite remap loop.
                state.typeahead.clear();
            } 
            Ok(op) => {
                match op {
                    InsertOp::Cancel => {
                        return normal(1);
                    }
                }
            }
        }
        // Stay in insert mode.
        insert()
    }
}
