use key::MultiKey;
use key::parse::parse;
use state_machine::StateMachine;
use maps::{normal_mode_map, insert_mode_map, pending_mode_map};

pub struct Vixi {
    machine: StateMachine<MultiKey>,
}

impl Vixi {
    pub fn new() -> Self {
        Vixi {
            machine: StateMachine::with_maps(
                normal_mode_map(),
                pending_mode_map(),
                insert_mode_map(),
            ),
        }
    }

    pub fn process(&mut self, keys: &str) {
        for key in parse(keys) {
            self.machine.process(key);
        }
    }

    pub fn mode(&self) -> &'static str {
        self.machine.mode()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_in_normal() {
        let mut vixi = Vixi::new();
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_insert() {
        let mut vixi = Vixi::new();
        vixi.process("i");
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn insert_text() {
        let mut vixi = Vixi::new();
        vixi.process("iasdf");
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn insert_to_normal() {
        let mut vixi = Vixi::new();
        vixi.process("i<esc>");
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn insert_text_then_normal() {
        let mut vixi = Vixi::new();
        vixi.process("ir<esc>");
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_op_pending() {
        let mut vixi = Vixi::new();
        vixi.process("d");
        assert_eq!("Pending", vixi.mode());
    }

    #[test]
    fn to_op_pending_with_count() {
        let mut vixi = Vixi::new();
        vixi.process("123d");
        assert_eq!("Pending", vixi.mode());
    }

    #[test]
    fn op_pending_to_normal() {
        let mut vixi = Vixi::new();
        vixi.process("d<esc>");
        assert_eq!("Normal", vixi.mode());
    }
}
