extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode, state};


fn main() {
    terminal::open("DUEL17", 80, 30);
    terminal::set(config::Window::empty().resizeable(true));

    terminal::print_xy(32, 0, "load state A\nand A 0b0001\nJZ SLOT1");


    let size = state::size();

    for y in 0..size.height {
        terminal::print_xy(0, y, format!("{:#06X}", y).as_ref());
    }


    terminal::refresh();
    for event in terminal::events() {
        match event {
            Event::Resize { width, height } => {
                terminal::print_xy(0, 0, &*&format!("Width: {}\nHeight: {}", width, height));
                terminal::refresh();
            }
            Event::Close |
            Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => break,
            _ => (),
        }
    }
    terminal::close();
}
