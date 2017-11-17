use disambiguation_map::Match;
use mode::*;
use mode_map::MapErr;
use op::NormalOp;
use state::State;
use typeahead::Parse;
use client;

impl<C, K> Transition<C, K> for NormalMode<K>
where
    C: client::Client,
    C: Clone,
    K: Ord,
    K: Copy,
    K: Parse,
{
    fn name(&self) -> &'static str {
        "Normal"
    }

    fn transition(&self, state: &mut State<C, K>) -> Mode<K> {
        match state.normal_mode_map.process(&mut state.typeahead) {
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
                        state.count = n;
                        return self.transition(state);
                    }
                    Match::PartialMatch => {
                        return recast_normal(self);
                    }
                    Match::NoMatch => {
                        // In Normal mode, unmatched typeahead gets dropped.
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
                    NormalOp::Cancel => {
                        state.cancel();
                    }
                    NormalOp::Insert => {
                        return insert();
                    }
                    NormalOp::ReplaceChar => {
                        // TODO
                    }
                    NormalOp::ReplaceMode => {
                        return replace();
                    }
                    NormalOp::Repeat => {
                        // TODO
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
