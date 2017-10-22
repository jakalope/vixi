extern crate termion;

mod disambiguation_map;
mod mode;
mod mode_map;
mod op;
mod ordered_vec_map;
mod state;

fn main() {
    let mut state = state::State::new();
    let mut normal_mode = mode::Mode::new(&mut state);
    loop {
        let insert_mode = normal_mode.process();
        normal_mode = insert_mode.process();
    }
}
