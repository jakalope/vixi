use mode::{Mode, normal, Transition};
use mode_map::ModeMap;
use op::{InsertOp, PendingOp, NormalOp};
use state::State;
use typeahead::{Parse, RemapType};
use serde_json::Value;
use client;

pub struct StateMachine<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    state: State<K>,
    mode: Mode<K>,
}

impl<K> StateMachine<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    pub fn new(
        client: Box<client::Client>,
        normal_map: ModeMap<K, NormalOp>,
        pending_map: ModeMap<K, PendingOp>,
        insert_map: ModeMap<K, InsertOp>,
    ) -> Self {
        StateMachine {
            state: State::new(client, normal_map, pending_map, insert_map),
            mode: normal(),
        }
    }

    pub fn process(&mut self, key: K) {
        self.state.put(key, RemapType::Remap);
        self.mode = self.mode.transition(&mut self.state);
    }

    pub fn mode(&self) -> &'static str {
        self.mode.name()
    }
}
