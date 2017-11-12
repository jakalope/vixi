use mode::{Mode, normal, Transition};
use mode_map::ModeMap;
use op::{InsertOp, PendingOp, NormalOp};
use state::{NumericMap, State};
use typeahead::RemapType;

pub struct StateMachine<K>
where
    K: Ord,
    K: Copy,
{
    state: State<K>,
    mode: Mode<K>,
}

impl<K> StateMachine<K>
where
    K: Ord,
    K: Copy,
{
    pub fn with_maps(
        numeric_map: Box<NumericMap<K>>,
        normal_map: ModeMap<K, NormalOp>,
        pending_map: ModeMap<K, PendingOp>,
        insert_map: ModeMap<K, InsertOp>,
    ) -> Self {
        StateMachine {
            state: State::with_maps(
                numeric_map,
                normal_map,
                pending_map,
                insert_map,
            ),
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
