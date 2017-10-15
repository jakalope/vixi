use state::State;
use termion::event::Key;
use op::{NormalOp, InsertOp};

pub struct NormalMode<'a> {
    state: &'a mut State<Key>,
}

pub struct InsertMode<'a> {
    state: &'a mut State<Key>,
}

// Use an enum for modes. This is preferable to a traits approach, since we know
// up front what all possible modes will be. We use the data type of each
// variant to specify the implementation details of that variant, even though
// all variants' data types hold only a reference to vixi's state.
//
// Ultimately, this architecture allows us to limit the methods available to
// vixi according to her current mode.
pub enum Mode<'a> {
    Normal(NormalMode<'a>),
    Insert(InsertMode<'a>),
}

impl<'a> Mode<'a> {
    // Manage mode-switching ops.
    pub fn process(self) -> Mode<'a> {
        match self {
            Mode::Normal(x) => x.process(),
            Mode::Insert(x) => x.process(),
        }
    }
}

impl<'a> NormalMode<'a> {
    pub fn process(self) -> Mode<'a> {
        match self.state.normal_mode_map.process(
            &mut self.state.typeahead,
        ) {
            None => {
                // In Normal mode, unmatched typeahead gets dropped.
                self.state.typeahead.clear();
            } 
            Some(op) => {
                match op {
                    NormalOp::Insert => {
                        return Mode::Insert(InsertMode { state: self.state });
                    }
                }
            }
        };
        // Stay in normal mode.
        return Mode::Normal(self);
    }
}

impl<'a> InsertMode<'a> {
    pub fn process(self) -> Mode<'a> {
        match self.state.insert_mode_map.process(
            &mut self.state.typeahead,
        ) {
            None => {
                // In Insert mode, unmatched typeahead gets passed to the editor.
                self.state.typeahead.clear();
            } 
            Some(op) => {
                match op {
                    InsertOp::Cancel => {
                        return Mode::Normal(NormalMode { state: self.state });
                    }
                }
            }
        }
        // Stay in insert mode.
        return Mode::Insert(self);
    }
}

impl<'a> Mode<'a> {
    pub fn new(state: &'a mut State<Key>) -> Self {
        Mode::Normal(NormalMode { state: state })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_from_normal() {
        let mut state = State::new();
        let normal_mode = Mode::new(&mut state);
        let insert_mode = normal_mode.process();
    }

    #[test]
    fn test_normal_from_insert() {
        let mut state = State::new();
        let normal_mode = Mode::new(&mut state);
        let insert_mode = normal_mode.process();
        let normal_mode = insert_mode.process();
    }
}
