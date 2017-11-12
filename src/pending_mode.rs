use mode::*;
use mode_map::MapErr;
use op::PendingOp;
use state::State;
use typeahead::Numeric;
use disambiguation_map::Match;

impl<K> PendingMode<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    fn next_mode(&self, count: i32) -> Mode<K> {
        match self.next_mode {
            NextMode::Insert => insert(),
            NextMode::Normal => normal(count),
        }
    }
}

impl<K> Transition<K> for PendingMode<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    fn name(&self) -> &'static str {
        "Pending"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        let mut count = 1;
        match state.typeahead.parse_decimal() {
            Match::FullMatch(n) => {
                count = n;
            }
            Match::PartialMatch => {
                return recast_pending(self);
            }
            Match::NoMatch => {}
        };
        match state.pending_mode_map.process(&mut state.typeahead) {
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
                    PendingOp::Cancel => {
                        // TODO drop back to normal mode; clear count.
                    }
                    PendingOp::Operator(o) => {
                        // TODO Perform operation over [motion].
                        return self.next_mode(count);
                    }
                    PendingOp::Motion(m) => {
                        // TODO Perform operation over [motion].
                        return self.next_mode(count);
                    }
                    PendingOp::Object(o) => {
                        // TODO Perform operation over [object].
                        return self.next_mode(count);
                    }
                }
            }
        };
        // Go back to whence you came.
        self.next_mode(count)
    }
}
