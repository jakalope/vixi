use disambiguation_map::Match;
use mode::*;
use mode_map::MapErr;
use op::NormalOp;
use state::State;
use typeahead::Numeric;

impl<K> Transition<K> for NormalMode<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    fn name(&self) -> &'static str {
        "Normal"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        let mut count = 1;
        match state.typeahead.parse_decimal() {
            Match::FullMatch(n) => {
                count = n;
            }
            Match::PartialMatch => {
                return recast_normal(self);
            }
            Match::NoMatch => {}
        };
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
                    NormalOp::Operator(o) => {
                        // TODO
                        // Enter operator pending mode.
                        return pending(NextMode::Normal, count);
                    }
                    NormalOp::Motion(m) => {
                        // TODO
                    }
                }
            }
        };
        // Stay in normal mode.
        normal(count)
    }
}
