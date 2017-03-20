extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode, state};

use std::fmt;

#[derive(Clone, Copy)]
enum Instruction {
    NOP,
    Load(Data, Register),
}
use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::NOP => write!(f, "{}", "NOP"),
            Instruction::Load(data, register) => write!(f, "load {}{}", data, register),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Data {
    State,
}
use Data::*;

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
use Register::*;

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn main() {
    terminal::open("DUEL17", 80, 30);
    terminal::set(config::Window::empty().resizeable(true));

    terminal::print_xy(32, 0, "load state A\nand A 0b0001\nJZ SLOT1");

    let size = state::size();

    let instructions = get_instructions();

    for y in 0..size.height {
        terminal::print_xy(0,
                           y,
                           format!("{:#04X}│{}", y, instructions[y as usize]).as_ref());
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

fn get_instructions() -> [Instruction; 256] {
    let mut result = [NOP; 256];

    result[2] = Load(State, A);
    result[5] = Load(State, C);
    result[8] = Load(State, F);

    result
}
