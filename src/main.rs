extern crate termion;

mod mode;
mod common;
mod ordered_vec_map;

use std::collections::VecDeque;
use termion::event::Key;

fn main() {
    let mut typeahead = VecDeque::<Key>::new();
    typeahead.push_back(Key::Char('i'));
    typeahead.push_back(Key::Esc);

    let normal_mode = mode::Mode::<mode::NormalMode>::new(&typeahead);
    let insert_mode = mode::Mode::<mode::InsertMode>::from(normal_mode);
    let normal_mode = mode::Mode::<mode::NormalMode>::from(insert_mode);
    normal_mode.echo("asdf");
}
