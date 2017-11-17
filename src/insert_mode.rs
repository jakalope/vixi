use mode::{insert, normal, InsertMode, Mode, Transition};
use mode_map::MapErr;
use op::InsertOp;
use state::State;
use typeahead::Parse;
use client;

impl<C, K> Transition<C, K> for InsertMode<K>
where
    C: client::Client,
    C: Clone,
    K: Ord,
    K: Copy,
    K: Parse,
{
    fn name(&self) -> &'static str {
        "Insert"
    }

    fn transition(&self, state: &mut State<C, K>) -> Mode<K> {
        match state.insert_mode_map.process(&mut state.typeahead) {
            Err(MapErr::NoMatch) => {
                // In Insert mode, unmatched typeahead gets inserted.
                // TODO respect self.replace_mode
                // TODO client.insert_newline() on <CR>
                let string = state.typeahead.parse_string();
                for c in string.chars() {
                    state.client.char(&state.view_id, c);
                }
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
