use mode::{Mode, normal, Transition};
use mode_map::ModeMap;
use op::{InsertOp, PendingOp, NormalOp};
use state::State;
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
    pub fn new() -> Self {
        StateMachine {
            state: State::<K>::new(),
            mode: normal(),
        }
    }

    pub fn with_maps(
        normal_map: ModeMap<K, NormalOp>,
        pending_map: ModeMap<K, PendingOp>,
        insert_map: ModeMap<K, InsertOp>,
    ) -> Self {
        StateMachine {
            state: State::with_maps(normal_map, pending_map, insert_map),
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

#[cfg(test)]
mod test {
    use key::Key;
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
        return map;
    }

    fn make_pending_mode_map() -> ModeMap<Key, PendingOp> {
        let mut map = ModeMap::new();
        return map;
    }

    fn make_insert_mode_map() -> ModeMap<Key, InsertOp> {
        let mut map = ModeMap::new();
        map.insert_op(vec![Key::Esc], InsertOp::Cancel);
        return map;
    }

    #[test]
    fn start_in_normal() {
        let vixi = StateMachine::<Key>::with_maps(
            make_normal_mode_map(),
            make_pending_mode_map(),
            make_insert_mode_map(),
        );
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn insert_from_normal() {
        let mut vixi = StateMachine::<Key>::with_maps(
            make_normal_mode_map(),
            make_pending_mode_map(),
            make_insert_mode_map(),
        );
        vixi.process(Key::Char('i'));
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn normal_from_insert() {
        let mut vixi = StateMachine::<Key>::with_maps(
            make_normal_mode_map(),
            make_pending_mode_map(),
            make_insert_mode_map(),
        );
        vixi.process(Key::Char('i'));
        vixi.process(Key::Esc);
        assert_eq!("Normal", vixi.mode());
    }
}
