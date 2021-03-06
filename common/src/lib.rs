extern crate rand;

use std::fmt;
use rand::{Rand, Rng, SeedableRng, StdRng};

pub struct Platform {
    pub print_xy: fn(i32, i32, &str),
    pub clear: fn(Option<Rect>),
    pub size: fn() -> Size,
    pub mouse_position: fn() -> Point,
    pub clicks: fn() -> i32,
    pub key_pressed: fn(KeyCode) -> bool,
    pub set_colors: fn(Color, Color),
}

pub const PLAYFIELD_SIZE: usize = 32;

pub struct Game {
    pub instructions: [Instruction; PLAYFIELD_SIZE],
    pub scroll_offset: i32,
    pub cards: Vec<Card>,
    pub selected_card: Option<usize>,
    pub playfield_right_edge: i32,
    pub ui_context: UIContext,
    pub run_button_spec: ButtonSpec,
    pub paused: bool,
    pub executing_address: Option<i32>,
    pub instruction_countdown: u16,
    pub registers: [u8; REGISTER_AMOUNT],
    pub rng: StdRng,
}

pub const REGISTER_AMOUNT: usize = 8;

pub fn to_register(n: i32) -> Option<Register> {
    match n {
        0 => Some(A),
        1 => Some(B),
        2 => Some(C),
        3 => Some(D),
        4 => Some(E),
        5 => Some(F),
        6 => Some(G),
        7 => Some(H),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Register {
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

pub const REGISTER_VARIATION_COUNT: u8 = 4;

impl Rand for Register {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, REGISTER_VARIATION_COUNT) {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            _ => H,
        }
    }
}

pub struct UIContext {
    pub hot: UiId,
    pub active: UiId, // pub interacting_with: UiId,
    pub next_hot: UiId,
}

impl UIContext {
    pub fn set_not_active(&mut self) {
        self.active = 0;
    }
    pub fn set_active(&mut self, id: UiId) {
        self.active = id;
    }
    pub fn set_next_hot(&mut self, id: UiId) {
        self.next_hot = id;
    }
    pub fn set_not_hot(&mut self) {
        self.hot = 0;
    }
    pub fn frame_init(&mut self) {
        if self.active == 0 {
            self.hot = self.next_hot;
        }
        self.next_hot = 0;
    }
}

pub type UiId = i32;

pub struct ButtonSpec {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub text: String,
}

pub struct Card {
    pub location: Point,
    pub instructions: Vec<Instruction>,
}

impl Card {
    pub fn new(x: i32, y: i32, instructions: Vec<Instruction>) -> Self {
        Card {
            location: Point::new(x, y),
            instructions: instructions,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Instruction {
    NOP,
    Load(Data, Register),
    Add(Data, Register),
    Sub(Data, Register),
    JumpZero(Data, Register),
    JumpNotZero(Data, Register),
    JumpRZero(Register, Register),
    JumpRNotZero(Register, Register),
}
use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::NOP => write!(f, "{}", "NOP"),
            Instruction::Load(data, register) => write!(f, "load {} {}", data, register),
            Instruction::Add(data, register) => write!(f, "add  {} {}", data, register),
            Instruction::Sub(data, register) => write!(f, "sub  {} {}", data, register),
            Instruction::JumpZero(data, register) => write!(f, "JZ  {} {}", data, register),
            Instruction::JumpNotZero(data, register) => write!(f, "JNZ  {} {}", data, register),
            Instruction::JumpRZero(register1, register2) => {
                write!(f, "JRZ {} {}", register1, register2)
            }
            Instruction::JumpRNotZero(register1, register2) => {
                write!(f, "JRNZ {} {}", register1, register2)
            }
        }
    }
}

pub const INSTRUCTION_VARIATION_COUNT: u8 = 8;

impl Rand for Instruction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, INSTRUCTION_VARIATION_COUNT) {
            1 => Load(rng.gen::<Data>(), rng.gen::<Register>()),
            2 => Add(rng.gen::<Data>(), rng.gen::<Register>()),
            3 => Sub(rng.gen::<Data>(), rng.gen::<Register>()),
            4 => JumpZero(rng.gen::<Data>(), rng.gen::<Register>()),
            5 => JumpNotZero(rng.gen::<Data>(), rng.gen::<Register>()),
            6 => JumpRZero(rng.gen::<Register>(), rng.gen::<Register>()),
            7 => JumpRNotZero(rng.gen::<Register>(), rng.gen::<Register>()),
            _ => NOP,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Data {
    Immeadiate(u8),
}
use Data::*;

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Immeadiate(value) => write!(f, "{:#04X}", value),
        }
    }
}

pub const DATA_VARIATION_COUNT: u8 = 1;

impl Rand for Data {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, DATA_VARIATION_COUNT) {
            _ => Immeadiate(rng.gen::<u8>()),

        }
    }
}


pub fn get_instructions() -> [Instruction; PLAYFIELD_SIZE] {
    let mut result = [NOP; PLAYFIELD_SIZE];

    result[2] = Load(Immeadiate(2), A);
    result[4] = Load(Immeadiate(4), B);
    result[8] = Load(Immeadiate(8), C);
    result[16] = Load(Immeadiate(16), D);
    // result[32] = Load(Immeadiate(32), E);
    // result[64] = Load(Immeadiate(64), F);
    // result[128] = Load(Immeadiate(128), G);
    // result[254] = Load(Immeadiate(254), H);

    result
}

impl Point {
    /// Creates a new point on the specified non-negative coordinates
    pub fn new_safe(mut x: i32, mut y: i32) -> Point {
        x = if x >= 0 { x } else { 0 };
        y = if y >= 0 { y } else { 0 };

        Point { x: x, y: y }
    }

    pub fn add(&self, x: i32, y: i32) -> Point {
        Point::new_safe(self.x + x, self.y + y)
    }
}
//if I import BearLibTerminal.rs into `game` or a crate `game` depends on,
//like this one for example, then the ffi to the C version of
//BearLibTerminal causes an error. I just want the geometry datatypes and
//the Event and Keycode definitions so I have copied them from the
//BearLibTerminal.rs geometry module and input module below.

//BearLibTerminal.rs is released under the MIT license by nabijaczleweli.
//see https://github.com/nabijaczleweli/BearLibTerminal.rs/blob/master/LICENSE
//for full details.

/// Represents a single on-screen point/coordinate pair.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point on the specified non-negative coordinates
    pub fn new(x: i32, y: i32) -> Point {
        assert!(x >= 0);
        assert!(y >= 0);

        Point { x: x, y: y }
    }
}


/// A 2D size representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Creates a new non-negative size.
    pub fn new(width: i32, height: i32) -> Size {
        assert!(width >= 0);
        assert!(height >= 0);

        Size {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}x{}", self.width, self.height)
    }
}

/// A rectangle, described by its four corners and a size.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
    /// The top-left corner.
    pub top_left: Point,
    /// The top-right corner.
    pub top_right: Point,
    /// The bottom-right corner.
    pub bottom_right: Point,
    /// The bottom-left corner.
    pub bottom_left: Point,
    /// The `Rect`angle's size.
    pub size: Size,
}

impl Rect {
    /// Construct a `Rect` from its top-left corner and its size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// let rect = Rect::from_size(Point::new(10, 20), Size::new(30, 40));
    /// assert_eq!(rect.top_left, Point::new(10, 20));
    /// assert_eq!(rect.top_right, Point::new(40, 20));
    /// assert_eq!(rect.bottom_left, Point::new(10, 60));
    /// assert_eq!(rect.bottom_right, Point::new(40, 60));
    /// assert_eq!(rect.size, Size::new(30, 40));
    /// ```
    pub fn from_size(origin: Point, size: Size) -> Rect {
        let top_right = Point::new(origin.x + size.width, origin.y);
        let bottom_left = Point::new(origin.x, origin.y + size.height);
        let bottom_right = Point::new(top_right.x, bottom_left.y);

        Rect {
            top_left: origin,
            top_right: top_right,
            bottom_left: bottom_left,
            bottom_right: bottom_right,
            size: size,
        }
    }

    /// Construct a `Rect` from its top-left and bottom-right corners.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// let rect = Rect::from_points(Point::new(10, 20), Point::new(30, 40));
    /// assert_eq!(rect.top_left, Point::new(10, 20));
    /// assert_eq!(rect.top_right, Point::new(30, 20));
    /// assert_eq!(rect.bottom_left, Point::new(10, 40));
    /// assert_eq!(rect.bottom_right, Point::new(30, 40));
    /// assert_eq!(rect.size, Size::new(20, 20));
    /// ```
    pub fn from_points(top_left: Point, bottom_right: Point) -> Rect {
        assert!(bottom_right.x >= top_left.x);
        assert!(bottom_right.y >= top_left.y);

        let size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
        Rect::from_size(top_left, size)
    }

    /// Construct a `Rect` from its top-left corner and its size, values unwrapped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// assert_eq!(Rect::from_values(10, 20, 30, 40),
    ///     Rect::from_size(Point::new(10, 20), Size::new(30, 40)));
    /// ```
    pub fn from_values(x: i32, y: i32, width: i32, height: i32) -> Rect {
        let origin = Point::new(x, y);
        let size = Size::new(width, height);
        Rect::from_size(origin, size)
    }


    /// Construct a `Rect` from its top-left and bottom-right corners, values unwrapped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// assert_eq!(Rect::from_point_values(10, 20, 30, 40),
    ///     Rect::from_points(Point::new(10, 20), Point::new(30, 40)));
    /// ```
    pub fn from_point_values(top_left_x: i32,
                             top_left_y: i32,
                             bottom_right_x: i32,
                             bottom_right_y: i32)
                             -> Rect {
        let top_left = Point::new(top_left_x, top_left_y);
        let bottom_right = Point::new(bottom_right_x, bottom_right_y);
        Rect::from_points(top_left, bottom_right)
    }
}

//input module

/// All pressable keys.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    /// Top-row `1/!` key.
    Row1,
    /// Top-row `2/@` key.
    Row2,
    /// Top-row `3/#` key.
    Row3,
    /// Top-row `4/$` key.
    Row4,
    /// Top-row `5/%` key.
    Row5,
    /// Top-row `6/^` key.
    Row6,
    /// Top-row `7/&` key.
    Row7,
    /// Top-row `8/*` key.
    Row8,
    /// Top-row `9/(` key.
    Row9,
    /// Top-row `0/)` key.
    Row0,
    /// Top-row &#96;/~ key.
    Grave,
    /// Top-row `-/_` key.
    Minus,
    /// Top-row `=/+` key.
    Equals,
    /// Second-row `[/{` key.
    LeftBracket,
    /// Second-row `]/}` key.
    RightBracket,
    /// Second-row `\/|` key.
    Backslash,
    /// Third-row `;/:` key.
    Semicolon,
    /// Third-row `'/"` key.
    Apostrophe,
    /// Fourth-row `,/<` key.
    Comma,
    /// Fourth-row `./>` key.
    Period,
    /// Fourth-row `//?` key.
    Slash,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    /// Right arrow key.
    Right,
    /// Left arrow key.
    Left,
    /// Down arrow key.
    Down,
    /// Up arrow key.
    Up,
    /// Numpad `/` key.
    NumDivide,
    /// Numpad `*` key.
    NumMultiply,
    /// Numpad `-` key.
    NumMinus,
    /// Numpad `+` key.
    NumPlus,
    /// Numpad &#9166; key.
    NumEnter,
    /// Numpad `Del/.` key (output locale-dependent).
    NumPeriod,
    /// Numpad `1/End` key.
    Num1,
    /// Numpad 2/&#8595; key.
    Num2,
    /// Numpad `3/PageDown` key.
    Num3,
    /// Numpad 4/&#8592; key.
    Num4,
    /// Numpad `5` key.
    Num5,
    /// Numpad 6/&#8594; key.
    Num6,
    /// Numpad `7/Home` key.
    Num7,
    /// Numpad 8/&#8593; key.
    Num8,
    /// Numpad `9/PageUp` key.
    Num9,
    /// Numpad `0/Insert` key.
    Num0,
    /// Left mouse button.
    MouseLeft,
    /// Right mouse button.
    MouseRight,
    /// Middle mouse button a.k.a. pressed scroll wheel.
    MouseMiddle,
    MouseFourth,
    MouseFifth,
}

/// A single input event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    /// Terminal window closed.
    Close,
    /// Terminal window resized. Needs to have `window.resizeable = true` to occur.
    ///
    /// Note, that the terminal window is cleared when resized.
    Resize {
        /// Width the terminal was resized to.
        width: i32,
        /// Heigth the terminal was resized to.
        height: i32,
    },
    /// Mouse moved.
    ///
    /// If [`precise-mouse`](config/struct.Input.html#structfield.precise_mouse) is off,
    /// generated each time mouse moves from cell to cell, otherwise,
    /// when it moves from pixel to pixel.
    MouseMove {
        /// `0`-based cell index from the left to which the mouse cursor moved.
        x: i32,
        /// `0`-based cell index from the top to which the mouse cursor moved.
        y: i32,
    },
    /// Mouse wheel moved.
    MouseScroll {
        /// Amount of steps the wheel rotated.
        ///
        /// Positive when scrolled "down"/"backwards".
        ///
        /// Negative when scrolled "up"/"forwards"/"away".
        delta: i32,
    },
    /// A keyboard or mouse button pressed (might repeat, if set in OS).
    KeyPressed {
        /// The key pressed.
        key: KeyCode,
        /// Whether the Control key is pressed.
        ctrl: bool,
        /// Whether the Shift key is pressed.
        shift: bool,
    },
    /// A keyboard or mouse button released.
    KeyReleased {
        /// The key released.
        key: KeyCode,
        /// Whether the Control key is pressed.
        ctrl: bool,
        /// Whether the Shift key is pressed.
        shift: bool,
    },
    /// The Shift key pressed (might repeat, if set in OS).
    ShiftPressed,
    /// The Shift key released.
    ShiftReleased,
    /// The Shift key pressed (might repeat, if set in OS).
    ControlPressed,
    /// The Control key released.
    ControlReleased,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
