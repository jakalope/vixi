extern crate termion;

mod common;
mod mode;
mod op;
mod ordered_vec_map;
mod state;
mod mode_map;

fn main() {
    let mut state = state::State::new();
    let mut normal_mode = mode::Mode::new(&mut state);
    loop {
        let insert_mode = normal_mode.process();
        normal_mode = insert_mode.process();
    }
}
