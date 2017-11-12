use disambiguation_map::Match;
use mode_map::ModeMap;
use op::{NormalOp, PendingOp, InsertOp};
use typeahead::{Numeric, RemapType, Typeahead};

pub struct State<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    pub typeahead: Typeahead<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub pending_mode_map: ModeMap<K, PendingOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
    pub count: i32, // Used when an op is to be performed [count] times.
}

impl<K> State<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
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
            count: 0,
        }
    }

    pub fn put(&mut self, key: K, remap_type: RemapType) {
        self.typeahead.push_back(key, remap_type);
    }

    // Clears state variables. Used when an <Esc> is encountered.
    pub fn cancel(&mut self) {
        self.count = 0;
        self.typeahead.clear();
    }
}
