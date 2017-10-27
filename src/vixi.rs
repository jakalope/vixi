use mode::{Mode,NormalMode,InsertMode,Transition};
use state::State;
use termion::event::Key;
use typeahead::RemapType;

pub struct Vixi {
    state: State,
    mode: Mode,    
}

impl Vixi {
    pub fn new() -> Self {
        Vixi {
            state: State::new(),
            mode: Mode::Normal(NormalMode{}),
        }
    }

    pub fn process(&mut self, key: Key, remap_type: RemapType) {
        self.state.put(key, remap_type);
        self.mode = self.mode.transition(&mut self.state);
    }

    pub fn mode(&self) -> &'static str {
        self.mode.name()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_in_normal() {
        let vixi = Vixi::new();
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn insert_from_normal() {
        let mut vixi = Vixi::new();
        vixi.process(Key::Char('i'), RemapType::Remap);
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn normal_from_insert() {
        let mut vixi = Vixi::new();
        vixi.process(Key::Char('i'), RemapType::Remap);
        vixi.process(Key::Esc, RemapType::Remap);
        assert_eq!("Normal", vixi.mode());
    }
}
