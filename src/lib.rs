extern crate termion;
extern crate serde_json;

mod disambiguation_map;
mod insert_mode;
mod mode;
mod mode_map;
mod normal_mode;
mod pending_mode;
mod op;
mod ordered_vec_map;
mod state;
mod typeahead;
mod state_machine;

pub mod vixi;
