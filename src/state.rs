use mode_map::ModeMap;
use op::{NormalOp, InsertOp};
use typeahead::{RemapType, Typeahead};

#[derive(Debug, PartialEq)]
pub struct State<K> where K: Ord, K: Copy {
    pub typeahead: Typeahead<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
}

impl<K> State<K> where K: Ord, K: Copy {
    pub fn new() -> Self {
        State {
            typeahead: Typeahead::<K>::new(),
            normal_mode_map: ModeMap::<K, NormalOp>::new(),
            insert_mode_map: ModeMap::<K, InsertOp>::new(),
        }
    }

    pub fn with_maps(normal_map: ModeMap<K, NormalOp>,
                     insert_map: ModeMap<K, InsertOp>) -> Self {
        State {
            typeahead: Typeahead::<K>::new(),
            normal_mode_map: normal_map,
            insert_mode_map: insert_map,
        }
    }

    pub fn put(&mut self, key: K, remap_type: RemapType) {
        self.typeahead.push_back(key, remap_type);
    }
}
