use disambiguation_map::Match;
use mode_map::ModeMap;
use op::{NormalOp, PendingOp, InsertOp};
use typeahead::{Parse, RemapType, Typeahead};
use serde_json::Value;
use std::mem::swap;

pub struct State<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    pub typeahead: Typeahead<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub pending_mode_map: ModeMap<K, PendingOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
    pub count: i32, // Used when an op is to be performed [count] times.

    // Outgoing JSON commands.
    outgoing: Vec<Value>,
}

impl<K> State<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    pub fn with_maps(
        normal_map: ModeMap<K, NormalOp>,
        pending_map: ModeMap<K, PendingOp>,
        insert_map: ModeMap<K, InsertOp>,
    ) -> Self {
        State {
            typeahead: Typeahead::<K>::new(),
            normal_mode_map: normal_map,
            pending_mode_map: pending_map,
            insert_mode_map: insert_map,
            count: 1,
            outgoing: Vec::new(),
        }
    }

    /// Add a key to the typeahead buffer for processing.
    pub fn put(&mut self, key: K, remap_type: RemapType) {
        self.typeahead.push_back(key, remap_type);
    }

    /// Clear state variables. Used when an `<Esc>` is encountered.
    pub fn cancel(&mut self) {
        self.count = 1;
        self.typeahead.clear();
    }

    /// Add an outgoing JSON object, to be consumed by a call to
    /// `outgoing()`.
    pub fn send(&mut self, s: Value) {
        self.outgoing.push(s);
    }

    /// Consume outgoing JSON objects, intended for publication to Xi.
    pub fn outgoing(&mut self) -> Vec<Value> {
        let mut v = Vec::new();
        swap(&mut self.outgoing, &mut v);
        return v;
    }
}
