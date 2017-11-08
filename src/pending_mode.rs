use mode::{insert, pending, normal, PendingMode, NextMode, Mode, Transition};
use mode_map::MapErr;
use op::PendingOp;
use state::State;

impl<K> PendingMode<K>
where
    K: Ord,
    K: Copy,
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
{
    fn name(&self) -> &'static str {
        "Pending"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
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
                    PendingOp::Count(n) => {
                        state.count = n;
                        return pending(self.next_mode);
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
