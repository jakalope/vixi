extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nom;

extern crate regex;

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
mod key;

pub mod vixi;
