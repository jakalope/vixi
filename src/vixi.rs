use client;
use key::MultiKey;
use key::parse::parse;
use state_machine::StateMachine;
use maps::{normal_mode_map, insert_mode_map, pending_mode_map};
use serde_json::Value;

pub struct Vixi<C>
where
    C: client::Client,
    C: Clone,
{
    machine: StateMachine<C, MultiKey>,
}

impl<C> Vixi<C>
where
    C: client::Client,
    C: Clone,
{
    pub fn new(client: C) -> Self {
        Vixi {
            machine: StateMachine::new(
                client,
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
        let mut vixi = Vixi::new(client::DummyClient::new());
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_insert() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("i");
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn insert_text() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("iasdf");
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn insert_to_normal() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("i<esc>");
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn insert_text_then_normal() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("ir<esc>");
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_op_pending() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("d");
        assert_eq!("Pending", vixi.mode());
    }

    #[test]
    fn to_op_pending_with_count() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("123d");
        assert_eq!("Pending", vixi.mode());
    }

    #[test]
    fn op_pending_to_normal() {
        let mut vixi = Vixi::new(client::DummyClient::new());
        vixi.process("d<esc>");
        assert_eq!("Normal", vixi.mode());
    }
}
