use mode_map::MapErr;
use op::{NormalOp, InsertOp};
use state::State;
use termion::event::Key;
use typeahead::RemapType;

#[derive(Debug, PartialEq)]
pub struct NormalMode {
    state: State,
}

#[derive(Debug, PartialEq)]
pub struct InsertMode {
    state: State,
}

// Use an enum for modes. This is preferable to a traits approach, since we know
// up front what all possible modes will be. We use the data type of each
// variant to specify the implementation details of that variant, even though
// all variants' data types hold only a reference to vixi's state.
//
// Ultimately, this architecture allows us to limit the methods available to
// vixi according to her current mode.
#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
}

impl Mode {
    pub fn new(state: State) -> Self {
        Mode::Normal(NormalMode { state: state })
    }

    // Manage mode-switching ops.
    pub fn put(&mut self, key: Key, remap_type: RemapType) {
        match self {
            &mut Mode::Normal(ref mut x) => x.put(key, remap_type),
            &mut Mode::Insert(ref mut x) => x.put(key, remap_type),
        }
    }

    pub fn process(self) -> Mode {
        // Manage mode-switching ops. A `process()` method blocks here until a
        // mode switch op is returned.
        match self {
            Mode::Normal(x) => x.process(),
            Mode::Insert(x) => x.process(),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Mode::Normal(_) => "Normal",
            Mode::Insert(_) => "Insert",
        }
    }
}

impl NormalMode {
    pub fn put(&mut self, key: Key, remap_type: RemapType) {
        self.state.put(key, remap_type);
    }

    pub fn process(mut self) -> Mode {
        match self.state.normal_mode_map.process(
            &mut self.state.typeahead,
        ) {
            Err(MapErr::NoMatch) => {
                // In Normal mode, unmatched typeahead gets dropped.
                self.state.typeahead.clear();
            } 
            Err(MapErr::InfiniteRecursion) => {
                // TODO Tell the user they've created an infinite remap loop.
                self.state.typeahead.clear();
            } 
            Ok(op) => {
                match op {
                    NormalOp::Insert => {
                        return Mode::Insert(InsertMode { state: self.state });
                    }
                    NormalOp::Delete => {
                        // TODO Enter operator pending mode.
                    }
                }
            }
        };
        // Stay in normal mode.
        return Mode::Normal(self);
    }
}

impl InsertMode {
    pub fn put(&mut self, key: Key, remap_type: RemapType) {
        self.state.put(key, remap_type);
    }

    pub fn process(mut self) -> Mode {
        match self.state.insert_mode_map.process(
            &mut self.state.typeahead,
        ) {
            Err(MapErr::NoMatch) => {
                // In Normal mode, unmatched typeahead gets dropped.
                self.state.typeahead.clear();
            } 
            Err(MapErr::InfiniteRecursion) => {
                // TODO Tell the user they've created an infinite remap loop.
                self.state.typeahead.clear();
            } 
            Ok(op) => {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_from_normal() {
        let mut mode = Mode::new(State::new());
        mode.put(Key::Char('i'), RemapType::Remap);
        mode = mode.process();
        assert_eq!("Insert", mode.name());
    }

    #[test]
    fn normal_from_insert() {
        let mut mode = Mode::new(State::new());
        mode.put(Key::Char('i'), RemapType::Remap);
        mode = mode.process();
        mode.put(Key::Esc, RemapType::Remap);
        mode = mode.process();
        assert_eq!("Normal", mode.name());
    }
}
