use tiny_rpg::main_loop;
use tiny_rpg::start;

fn main() {
    let init = start::main_start().unwrap();
    main_loop(init).unwrap();
}
