extern crate libloading;
extern crate bear_lib_terminal_sys;
extern crate bear_lib_terminal;
extern crate common;

use libloading::Library;

use bear_lib_terminal::terminal::{self, config, Event, KeyCode, state};
use bear_lib_terminal::geometry::{Point, Rect, Size};

use std::mem;

use common::*;
use common::Register::*;
use common::Data::*;
use common::Instruction::*;

const LIB_PATH: &'static str = "./target/debug/libgame.so";

struct Application(Library);
impl Application {
    fn clamp_scroll_offset(&self, scroll_offset: i32) -> i32 {
        unsafe {
            let f = self.0.get::<fn(i32) -> i32>(b"clamp_scroll_offset\0").unwrap();
            f(scroll_offset)
        }
    }
    fn get_instructions(&self) -> [Instruction; common::PLAYFIELD_SIZE] {
        unsafe {
            let f = self.0
                .get::<fn() -> [Instruction; common::PLAYFIELD_SIZE]>(b"get_instructions\0")
                .unwrap();
            f()
        }
    }
    fn draw(&self,
            platform: &Platform,
            instructions: [Instruction; common::PLAYFIELD_SIZE],
            scroll_offset: i32) {
        unsafe {
            let f = self.0
                .get::<fn(&Platform, [Instruction; common::PLAYFIELD_SIZE], i32)>(b"draw\0")
                .unwrap();
            f(platform, instructions, scroll_offset)
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

    let instructions = app.get_instructions();

    let mut scroll_offset: i32 = 0;

    let platform = Platform {
        print_xy: terminal::print_xy,
        clear: clear,
        size: size,
    };

    app.draw(&platform, instructions, scroll_offset);

    terminal::refresh();

    loop {

        if let Some(event) = terminal::read_event() {
            match event {
                Event::MouseScroll { delta } => {
                    scroll_offset = scroll_offset.saturating_add(delta);
                }
                Event::KeyPressed { key: KeyCode::Up, ctrl: _, shift: _ } => {
                    scroll_offset = app.clamp_scroll_offset(scroll_offset.saturating_add(-1));
                }
                Event::KeyPressed { key: KeyCode::Down, ctrl: _, shift: _ } => {
                    scroll_offset = app.clamp_scroll_offset(scroll_offset.saturating_add(1));
                }
                Event::Close |
                Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => break,
                _ => (),
            }
        }

        terminal::clear(None);

        app.draw(&platform, instructions, scroll_offset);

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

fn clear(area: Option<common::Rect>) {

    match area {
        Some(rect) => {
            bear_lib_terminal_sys::clear_area(rect.top_left.x,
                                              rect.top_left.y,
                                              rect.size.width,
                                              rect.size.height)
        }
        None => bear_lib_terminal_sys::clear(),
    }

    //switch to this when/if my pull request is published
    // unsafe { terminal::clear(mem::transmute::<Option<common::Rect>, Option<Rect>>(area)) };
}

fn size() -> common::Size {
    unsafe { mem::transmute::<Size, common::Size>(state::size()) }
}
