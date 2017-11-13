use mode::*;
use mode_map::MapErr;
use op::PendingOp;
use state::State;
use typeahead::Parse;
use disambiguation_map::Match;

impl<K> PendingMode<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    fn next_mode(&self) -> Mode<K> {
        match self.next_mode {
            NextMode::Insert => insert(),
            NextMode::Normal => normal(),
        }
    }
}

impl<K> Transition<K> for PendingMode<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    fn name(&self) -> &'static str {
        "Pending"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match state.pending_mode_map.process(&mut state.typeahead) {
            Err(MapErr::NoMatch) => {
                // In vim, if one remaps a numeric, e.g.
                //   nnoremap 123 iasdf<Esc>
                // and proceeds to type 1234, the remap does not wait for
                // a disambiguating keystroke before completing the remap.
                // By putting parse_decimal() here instead of in
                // ModeMap::process(), we mimic this behavior.
                match state.typeahead.parse_decimal() {
                    Match::FullMatch(n) => {
                        // Update count and stay in same mode.
                        state.count *= n;
                        return self.transition(state);
                    }
                    Match::PartialMatch => {
                        return recast_pending(self);
                    }
                    Match::NoMatch => {
                        // In Pending mode, unmatched typeahead gets dropped.
                        state.typeahead.clear();
                    }
                };
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
                        return self.next_mode();
                    }
                    PendingOp::Motion(m) => {
                        // TODO Perform operation over [motion].
                        return self.next_mode();
                    }
                    PendingOp::Object(o) => {
                        // TODO Perform operation over [object].
                        return self.next_mode();
                    }
                }
            }
        };
        // Go back to whence you came.
        self.next_mode()
    }
}
