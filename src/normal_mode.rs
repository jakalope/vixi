use mode::{insert, normal, NormalMode, Mode, Transition};
use mode_map::MapErr;
use op::NormalOp;
use state::State;
use typeahead::RemapType;

impl Transition for NormalMode {
    fn name(&self) -> &'static str {
        "Normal"
    }

    fn transition(&self, state: &mut State) -> Mode {
        match state.normal_mode_map.process(
            &mut state.typeahead,
        ) {
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
                    NormalOp::Insert => {
                        return insert();
                    }
                    NormalOp::Delete => {
                        // TODO Enter operator pending mode.
                    }
                }
            }
        };
        // Stay in normal mode.
        normal()
    }
}

