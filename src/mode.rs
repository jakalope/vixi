use state::State;
use termion::event::Key;

pub struct NormalMode<'a> {
    state: &'a State<Key>,
}

pub struct InsertMode<'a> {
    state: &'a State<Key>,
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
        // Simplified during design phase. Ultimately, each mode's process()
        // method will be it's way of interacting with Vixi's state.
        Mode::Insert(InsertMode { state: self.state })
    }
}

impl<'a> InsertMode<'a> {
    pub fn process(self) -> Mode<'a> {
        // Simplified during design phase. Ultimately, each mode's process()
        // method will be it's way of interacting with Vixi's state.
        Mode::Normal(NormalMode { state: self.state })
    }
}

impl<'a> Mode<'a> {
    pub fn new(state: &'a State<Key>) -> Self {
        Mode::Normal(NormalMode { state: state })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_from_normal() {
        let state = State::new();
        let normal_mode = Mode::new(&state);
        let insert_mode = normal_mode.process();
    }

    #[test]
    fn test_normal_from_insert() {
        let state = State::new();
        let normal_mode = Mode::new(&state);
        let insert_mode = normal_mode.process();
        let normal_mode = insert_mode.process();
    }
}
