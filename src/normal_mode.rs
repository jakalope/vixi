use disambiguation_map::Match;
use mode::{NextMode, pending, insert, normal, NormalMode, Mode, Transition};
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
        match state.numeric_map.process(&mut state.typeahead) {
            Match::PartialMatch => {
                // Waiting for numeric to finish being matched.
                return normal();
            }
            Match::FullMatch(n) => {
                state.count = n;
            }
            Match::NoMatch => {}
        }
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
                        state.cancel();
                    }
                    NormalOp::Insert => {
                        return insert();
                    }
                    NormalOp::Repeat => {
                        // TODO
                    }
                    NormalOp::Count(n) => {
                        state.count = n;
                        return normal();
                    }
                    NormalOp::Operator(o) => {
                        // TODO
                        // Enter operator pending mode.
                        return pending(NextMode::Normal);
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
