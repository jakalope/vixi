extern crate termion;

mod state;
mod mode;
mod common;
mod ordered_vec_map;

fn main() {
    let state = state::State::new();
    let mut normal_mode = mode::Mode::new(&state);
    loop {
        let insert_mode = normal_mode.process();
        normal_mode = insert_mode.process();
    }
}
