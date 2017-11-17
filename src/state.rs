use disambiguation_map::Match;
use mode_map::ModeMap;
use op::{NormalOp, PendingOp, InsertOp};
use typeahead::{Parse, RemapType, Typeahead};
use serde_json::Value;
use std::mem::swap;
use client;

pub struct State<C, K>
where
    C: client::Client,
    C: Clone,
    K: Ord,
    K: Copy,
    K: Parse,
{
    pub client: C,
    pub typeahead: Typeahead<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub pending_mode_map: ModeMap<K, PendingOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
    pub count: i32, // Used when an op is to be performed [count] times.
    pub view_id: String,
}

impl<C, K> State<C, K>
where
    C: client::Client,
    C: Clone,
    K: Ord,
    K: Copy,
    K: Parse,
{
    pub fn new(
        client: C,
        normal_map: ModeMap<K, NormalOp>,
        pending_map: ModeMap<K, PendingOp>,
        insert_map: ModeMap<K, InsertOp>,
    ) -> Self {
        State {
            client: client,
            typeahead: Typeahead::<K>::new(),
            normal_mode_map: normal_map,
            pending_mode_map: pending_map,
            insert_mode_map: insert_map,
            count: 1,
            view_id: String::new(),
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
}
