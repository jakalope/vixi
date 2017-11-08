use mode::{insert, normal, NormalMode, Mode, Transition};
use mode_map::MapErr;
use op::NormalOp;
use state::State;

impl<K> Transition<K> for NormalMode<K>
where
    K: Ord,
    K: Copy,
{
    fn name(&self) -> &'static str {
        "Normal"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match state.normal_mode_map.process(&mut state.typeahead) {
            Err(MapErr::NoMatch) => {
                // In Normal mode, unmatched typeahead gets dropped.
                state.typeahead.clear();
            } 
            Err(MapErr::InfiniteRecursion) => {
                // TODO Tell the user they've created an infinite remap loop.
                state.typeahead.clear();
            } 
            Ok(op) => {
                match op {
                    NormalOp::Cancel => {
                        // TODO clear various members of state
                    }
                    NormalOp::Insert => {
                        return insert();
                    }
                    NormalOp::Delete => {
                        // TODO Enter operator pending mode.
                    }
                    NormalOp::Repeat => {
                        // TODO
                    }
                    NormalOp::Count(n) => {
                        state.count = n;
                        return normal();
                    }
                    NormalOp::Motion(m) => {
                        // TODO
                    }
                }
            }
        };
        // Stay in normal mode.
        normal()
    }
}
