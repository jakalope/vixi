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

