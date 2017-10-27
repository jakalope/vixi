use mode_map::MapErr;
use op::{NormalOp, InsertOp};
use state::State;
use typeahead::RemapType;

pub trait Transition {
    fn name(&self) -> &'static str;
    fn transition(&self, state: &mut State) -> Mode;
}

#[derive(Clone, Copy, Debug)]
pub struct NormalMode { }

#[derive(Clone, Copy, Debug)]
pub struct InsertMode { }

pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
}

pub fn normal() -> Mode {
    Mode::Normal(NormalMode{})
}

pub fn insert() -> Mode {
    Mode::Insert(InsertMode{})
}

impl Transition for Mode {
    fn name(&self) -> &'static str {
        match *self {
            Mode::Normal(x) => x.name(),
            Mode::Insert(x) => x.name(),
        }
    }

    fn transition(&self, state: &mut State) -> Mode {
        match *self {
            Mode::Normal(x) => x.transition(state),
            Mode::Insert(x) => x.transition(state),
        }
    }
}

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

impl Transition for InsertMode {
    fn name(&self) -> &'static str {
        "Insert"
    }

    fn transition(&self, state: &mut State) -> Mode {
        match state.insert_mode_map.process(
            &mut state.typeahead,
        ) {
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
                        return normal();
                    }
                }
            }
        }
        // Stay in insert mode.
        insert()
    }
}
