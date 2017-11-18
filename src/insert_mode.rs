use mode::{insert, normal, InsertMode, Mode, Transition};
use mode_map::MapErr;
use op::InsertOp;
use state::State;
use typeahead::Parse;
use serde_json::Value;

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
                // TODO respect self.replace_mode
                let string = state.typeahead.parse_string();
                for c in string.chars() {
                    // TODO handle failure
                    let result = match c {
                        '\n' | '\r' => {
                            state.client.insert_newline(&state.view_id)
                        }
                        _ => state.client.char(&state.view_id, c),
                    };
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
                    // TODO handle errors
                    InsertOp::Up => {
                        state.client.up(&state.view_id);
                    }
                    InsertOp::Down => {
                        state.client.down(&state.view_id);
                    }
                    InsertOp::Left => {
                        state.client.left(&state.view_id);
                    }
                    InsertOp::Right => {
                        state.client.right(&state.view_id);
                    }
                    InsertOp::PageUp => {
                        state.client.page_up(&state.view_id);
                    }
                    InsertOp::PageDown => {
                        state.client.page_down(&state.view_id);
                    }
                    InsertOp::Backspace => {
                        state.client.backspace(&state.view_id);
                    }
                    InsertOp::Delete => {
                        state.client.delete(&state.view_id);
                    }
                    InsertOp::DeleteWord => {
                        // TODO Delete backwards till whitespace or beginning of
                        // line.
                        state.client.down(&state.view_id);
                    }
                    InsertOp::DeleteLine => {
                        // TODO Delete backwards till beginning of line.
                        state.client.down(&state.view_id);
                    }
                    InsertOp::Tab => {
                        state.client.char(&state.view_id, '\t');
                    }
                    InsertOp::Digraph => {
                        // TODO
                    }
                    InsertOp::InsertRegister => {
                        // TODO
                    }
                    InsertOp::InsertRegisterContents => {
                        // TODO
                    }
                }
            }
        }
        // Stay in insert mode.
        insert()
    }
}
