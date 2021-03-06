extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nom;

extern crate regex;
extern crate xrl;
extern crate futures;

mod client;
mod disambiguation_map;
mod insert_mode;
mod key;
mod maps;
mod mode;
mod mode_map;
mod normal_mode;
mod op;
mod ordered_vec_map;
mod pending_mode;
mod state;
mod state_machine;
mod typeahead;


pub mod vixi;
