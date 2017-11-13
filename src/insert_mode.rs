use mode::{insert, normal, InsertMode, Mode, Transition};
use mode_map::MapErr;
use op::InsertOp;
use state::State;
use typeahead::Parse;

impl<K> Transition<K> for InsertMode<K>
where
    K: Ord,
    K: Copy,
    K: Parse,
{
    fn name(&self) -> &'static str {
        "Insert"
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match state.insert_mode_map.process(&mut state.typeahead) {
            Err(MapErr::NoMatch) => {
                // In Insert mode, unmatched typeahead gets inserted.
                // TODO send string to Xi.
                println!("{}", state.typeahead.parse_string());
            } 
            Err(MapErr::InfiniteRecursion) => {
                // TODO Tell the user they've created an infinite remap loop.
                state.typeahead.clear();
            } 
            Ok(op) => {
                match op {
                    InsertOp::Cancel => {
                        return normal();
                    }
                    InsertOp::Quit => {
                        state.cancel();
                        return normal();
                    }
                    // TODO
                    InsertOp::Up => {}
                    InsertOp::Down => {}
                    InsertOp::Left => {}
                    InsertOp::Right => {}
                    InsertOp::PageUp => {}
                    InsertOp::PageDown => {}
                    InsertOp::Backspace => {}
                    InsertOp::Delete => {}
                    InsertOp::DeleteWord => {}
                    InsertOp::DeleteLine => {}
                    InsertOp::Tab => {}
                    InsertOp::Digraph => {}
                    InsertOp::InsertRegister => {}
                    InsertOp::InsertRegisterContents => {}
                }
            }
        }
        // Stay in insert mode.
        insert()
    }
}
