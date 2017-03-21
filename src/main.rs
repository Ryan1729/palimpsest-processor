extern crate libloading;
extern crate bear_lib_terminal;

use bear_lib_terminal::terminal::{self, config, Event, KeyCode, state};
use bear_lib_terminal::geometry::{Point, Rect, Size};

use std::fmt;


use libloading::Library;

const LIB_PATH: &'static str = "./target/debug/libgame.so";

struct Application(Library);
impl Application {
    fn get_message(&self) -> &'static str {
        unsafe {
            let f = self.0.get::<fn() -> &'static str>(b"get_message\0").unwrap();
            f()
        }
    }
}

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
#[allow(dead_code)]
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

macro_rules! clamp {
    ( $min : expr, $input: expr, $max: expr ) => {

        if $input < $min {
            $min
        } else if $input > $max {
            $max
        } else {
            $input
        }

    }
}

fn main() {
    terminal::open("DUEL17", 80, 30);
    terminal::set(config::Window::empty().resizeable(true));
    terminal::set(vec![config::InputFilter::Group {
                           group: config::InputFilterGroup::Keyboard,
                           both: false,
                       },
                       config::InputFilter::Group {
                           group: config::InputFilterGroup::Mouse,
                           both: false,
                       }]);

    let mut app = Application(Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)));

    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    let dur = std::time::Duration::from_secs(1);

    let instructions = get_instructions();

    let mut scroll_offset: i32 = 0;

    draw(instructions, scroll_offset);

    terminal::refresh();
    for event in terminal::events() {
        match event {
            Event::MouseScroll { delta } => {
                scroll_offset = scroll_offset.saturating_add(delta);
            }
            Event::KeyPressed { key: KeyCode::Up, ctrl: _, shift: _ } => {
                scroll_offset = clamp_scroll_offset(scroll_offset.saturating_add(-1));
            }
            Event::KeyPressed { key: KeyCode::Down, ctrl: _, shift: _ } => {
                scroll_offset = clamp_scroll_offset(scroll_offset.saturating_add(1));
            }
            Event::Close |
            Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => break,
            _ => (),
        }

        terminal::clear(None);

        terminal::print_xy(32, 0, app.get_message());

        draw(instructions, scroll_offset);

        terminal::refresh();



        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH).map(|m| m.modified()) {
            if modified > last_modified {
                drop(app);
                app =
                    Application(Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)));
                last_modified = modified;
            }
        }

    }
    terminal::close();
}

fn draw(instructions: [Instruction; PLAYFIELD_SIZE], scroll_offset: i32) {
    draw_instructions(instructions, scroll_offset);

    let size = state::size();

    draw_rect(12, size.height - 24, 12, 16);


}

fn draw_rect(x: i32, y: i32, w: i32, h: i32) {
    terminal::clear(Some(Rect::from_values(x, y, w, h)));

    let right = x + w;
    let bottom = y + h;
    // top
    terminal::print_xy(x, y, "┌");
    for i in (x + 1)..right {
        terminal::print_xy(i, y, "─");
    }
    terminal::print_xy(right, y, "┐");

    // sides
    for i in (y + 1)..bottom {
        terminal::print_xy(x, i, "│");
        terminal::print_xy(right, i, "│");
    }

    //bottom
    terminal::print_xy(x, bottom, "└");
    for i in (x + 1)..right {
        terminal::print_xy(i, bottom, "─");
    }
    terminal::print_xy(right, bottom, "┘");
}

fn clamp_scroll_offset(scroll_offset: i32) -> i32 {
    let height = state::size().height;
    let len = PLAYFIELD_SIZE as i32;

    clamp!(-height + 1, scroll_offset, len - 1)

}

const PLAYFIELD_SIZE: usize = 32;

fn draw_instructions(instructions: [Instruction; PLAYFIELD_SIZE], mut scroll_offset: i32) {
    scroll_offset = clamp_scroll_offset(scroll_offset);

    for y in 0..state::size().height {
        let address = y + scroll_offset;
        if let Some(instruction) = instructions.get(address as usize) {
            terminal::print_xy(0, y, format!("{:#04X}│{}", address, instruction).as_ref());

        } else if address == -1 {
            terminal::print_xy(0, y, "────┐");
        } else if address == PLAYFIELD_SIZE as i32 {
            terminal::print_xy(0, y, "────┘");
        } else {
            //don't print anything
        }
    }
}

fn get_instructions() -> [Instruction; PLAYFIELD_SIZE] {
    let mut result = [NOP; PLAYFIELD_SIZE];

    result[2] = Load(State, A);
    result[4] = Load(State, B);
    result[8] = Load(State, C);
    result[16] = Load(State, D);
    // result[32] = Load(State, E);
    // result[64] = Load(State, F);
    // result[128] = Load(State, G);
    // result[254] = Load(State, H);

    result
}
