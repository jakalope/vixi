use mode::{Mode,normal,Transition};
use mode_map::ModeMap;
use op::{InsertOp, NormalOp};
use state::State;
use typeahead::RemapType;

pub struct Vixi<K> where K: Ord, K: Copy {
    state: State<K>,
    mode: Mode<K>,    
}

impl<K> Vixi<K> where K: Ord, K: Copy {
    pub fn new() -> Self {
        Vixi {
            state: State::<K>::new(),
            mode: normal(),
        }
    }

    pub fn with_maps(normal_map: ModeMap<K, NormalOp>,
                     insert_map: ModeMap<K, InsertOp>) -> Self {
        Vixi {
            state: State::with_maps(normal_map, insert_map),
            mode: normal(),
        }
    }

    pub fn process(&mut self, key: K, remap_type: RemapType) {
        self.state.put(key, remap_type);
        self.mode = self.mode.transition(&mut self.state);
    }

    pub fn mode(&self) -> &'static str {
        self.mode.name()
    }
}

#[cfg(test)]
mod test {
    use termion::event::Key;
    use mode_map::*;
    use super::*;
    use mode::*;
    use op::*;

    fn str_to_keyvec(s: &str) -> Vec<Key> {
        let mut v = Vec::<Key>::with_capacity(s.len());
        for c in s.chars() {
            v.push(Key::Char(c));
        }
        return v;
    }

    fn make_normal_mode_map() -> ModeMap<Key, NormalOp> {
        let mut map = ModeMap::new();
        map.insert_op(str_to_keyvec("i"), NormalOp::Insert);
        map.insert_op(str_to_keyvec("d"), NormalOp::Delete);
        return map;
    }

    fn make_insert_mode_map() -> ModeMap<Key, InsertOp> {
        let mut map = ModeMap::new();
        map.insert_op(vec![Key::Esc], InsertOp::Cancel);
        return map;
    }

    #[test]
    fn start_in_normal() {
        let vixi = Vixi::<Key>::with_maps(make_normal_mode_map(),
                                   make_insert_mode_map());
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn insert_from_normal() {
        let mut vixi = Vixi::<Key>::with_maps(make_normal_mode_map(),
                                       make_insert_mode_map());
        vixi.process(Key::Char('i'), RemapType::Remap);
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn normal_from_insert() {
        let mut vixi = Vixi::<Key>::with_maps(make_normal_mode_map(),
                                       make_insert_mode_map());
        vixi.process(Key::Char('i'), RemapType::Remap);
        vixi.process(Key::Esc, RemapType::Remap);
        assert_eq!("Normal", vixi.mode());
    }
}
