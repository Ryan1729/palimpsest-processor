extern crate common;
extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

use common::*;
use common::Register::*;
use common::Data::*;
use common::Instruction::*;

#[no_mangle]
pub fn new_game(instructions: [Instruction; PLAYFIELD_SIZE], size: Size) -> Game {

    let seed: &[_] = &[42];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let cards = get_cards(&mut rng, size.height);

    let run_button_width = 11;
    let run_button_spec = ButtonSpec {
        x: size.width - (run_button_width + 1),
        y: 4,
        w: run_button_width,
        h: 3,
        text: "Run".to_string(),
    };

    Game {
        instructions: instructions,
        scroll_offset: 0,
        cards: cards,
        selected_card: None,
        playfield_right_edge: 16,
        ui_context: UIContext {
            hot: 0,
            active: 0,
            next_hot: 0,
        },
        run_button_spec: run_button_spec,
        executing_address: None,
        instruction_countdown: COUNTDOWN_LENGTH,
        registers: [0; REGISTER_AMOUNT],
        rng: rng,
    }
}


fn get_cards(rng: &mut StdRng, height: i32) -> Vec<Card> {

    let mut instructions_vector = vec![];

    for _ in 0..5 {
        let instruction_count = rng.gen_range::<u8>(1, 4);

        let mut instructions = vec![];

        for i in 0..instruction_count {
            instructions.push(rng.gen::<Instruction>())
        }


        instructions_vector.push(instructions);
    }

    make_hand(height, instructions_vector)
}

const COUNTDOWN_LENGTH: u16 = 60;
const COUNTDOWN_NOP_LENGTH: u16 = 10;

const CARD_OFFSET: i32 = 12;
const CARD_OFFSET_DELTA: i32 = 12;

fn make_hand(height: i32, instructions_list: Vec<Vec<Instruction>>) -> Vec<Card> {
    let mut result = Vec::new();

    let mut offset = CARD_OFFSET;
    for instructions in instructions_list {
        result.push(Card::new(offset, hand_height(height), instructions));

        offset += CARD_OFFSET_DELTA;
    }

    result
}

pub fn hand_height(height: i32) -> i32 {
    height - HAND_HEIGHT_OFFSET
}

const HAND_HEIGHT_OFFSET: i32 = 8;

fn collect_hand(cards: &mut Vec<Card>) {
    let mut offset = CARD_OFFSET;
    for card in cards.iter_mut() {
        card.location.x = offset;
        offset += CARD_OFFSET_DELTA;
    }
}

#[no_mangle]
//returns true if quit requested
pub fn update_and_render(platform: &Platform, game: &mut Game, events: &mut Vec<Event>) -> bool {
    let mut left_mouse_pressed = false;
    let mut left_mouse_released = false;

    for event in events {

        match *event {
            Event::MouseScroll { delta } => {
                game.scroll_offset = game.scroll_offset.saturating_add(delta);
            }
            Event::KeyPressed { key: KeyCode::MouseLeft, ctrl: _, shift: _ } => {
                left_mouse_pressed = true;

                let mouse_pos = (platform.mouse_position)();
                if let Some(index) = game.selected_card {

                    if let Some(address) = over_address(game, mouse_pos) {
                        let instructions = game.cards.remove(index).instructions;

                        let mut current_address = address;
                        for i in 0..instructions.len() {
                            game.instructions[current_address] = instructions[i];
                            current_address += 1;
                        }

                        collect_hand(&mut game.cards);
                    }

                    game.selected_card = None;
                } else {
                    game.selected_card = clicked_card(game, mouse_pos);
                }
            }
            Event::KeyReleased { key: KeyCode::MouseLeft, ctrl: _, shift: _ } => {
                left_mouse_released = true;
            }
            Event::KeyPressed { key: KeyCode::MouseRight, ctrl: _, shift: _ } => {
                game.selected_card = None;
            }
            Event::KeyPressed { key: KeyCode::Up, ctrl: _, shift: _ } => {
                game.scroll_offset = game.scroll_offset.saturating_add(-1);
            }
            Event::KeyPressed { key: KeyCode::Down, ctrl: _, shift: _ } => {
                game.scroll_offset = game.scroll_offset.saturating_add(1);
            }
            Event::KeyPressed { key: KeyCode::R, ctrl: true, shift: _ } => {
                *game = new_game(common::get_instructions(), (platform.size)());
            }
            Event::Resize { width, height } => {
                for card in game.cards.iter_mut() {
                    card.location.y = hand_height(height);
                }
            }
            Event::Close |
            Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => return true,
            _ => (),
        }
    }

    if game.cards.len() <= 0 {
        let height = (platform.size)().height;
        game.cards = get_cards(&mut game.rng, height);
    }

    if let Some(address) = game.executing_address {
        game.instruction_countdown -= 1;

        if game.instruction_countdown <= 0 {
            let new_address = execute(game, address);

            if is_on_playfield(new_address) {
                set_executing_address(game, new_address);
            } else {
                game.executing_address = None;
            }
        }

    }

    if game.selected_card.is_some() {
        game.ui_context.hot = CARD_UI_ID;
        game.ui_context.active = CARD_UI_ID;
    } else {
        if game.ui_context.hot == CARD_UI_ID {
            game.ui_context.set_not_hot();
            game.ui_context.set_not_active();
        }
    }

    game.ui_context.frame_init();

    if do_button(platform,
                 &mut game.ui_context,
                 &game.run_button_spec,
                 -704788405,
                 left_mouse_pressed,
                 left_mouse_released) {
        set_executing_address(game, 0);
    }

    let test_spec = ButtonSpec {
        x: game.run_button_spec.x,
        y: 8,
        w: game.run_button_spec.w,
        h: 3,
        text: "Break".to_string(),
    };

    if do_button(platform,
                 &mut game.ui_context,
                 &test_spec,
                 -804788405,
                 left_mouse_pressed,
                 left_mouse_released) {
        game.executing_address = None;
        game.instruction_countdown = COUNTDOWN_LENGTH;
    }

    (platform.print_xy)(32, 14, &format!("{:?}", game.executing_address));

    draw(platform, game);

    false
}

fn get_value(data: Data) -> u8 {
    match data {
        Immeadiate(v) => v,
    }
}

fn execute(game: &mut Game, address: i32) -> i32 {
    let instruction = get_instruction(game, address);

    match instruction {
        Load(data, register) => {
            let value = get_value(data);

            set_register(game, value, register);
        }
        Add(data, register) => {
            let value = get_value(data);

            let new_value = value.wrapping_add(get_register_value(game, register));

            set_register(game, new_value, register);
        }
        Sub(data, register) => {
            let value = get_value(data);

            let new_value = value.wrapping_sub(get_register_value(game, register));

            set_register(game, new_value, register);
        }
        NOP => {}
    }

    address + 1
}

fn set_register(game: &mut Game, value: u8, register: Register) {
    game.registers[register as usize] = value;
}
fn get_register_value(game: &mut Game, register: Register) -> u8 {
    game.registers[register as usize]
}

fn set_executing_address(game: &mut Game, new_address: i32) {
    if is_on_playfield(new_address) {
        game.executing_address = Some(new_address);

        let instruction = get_instruction(game, new_address);

        if instruction == NOP {
            game.instruction_countdown = COUNTDOWN_NOP_LENGTH;
        } else {
            game.instruction_countdown = COUNTDOWN_LENGTH;
        }

    }

}


fn get_instruction(game: &Game, address: i32) -> Instruction {
    game.instructions[address as usize]
}

fn is_on_playfield(new_address: i32) -> bool {
    new_address >= 0 && new_address < PLAYFIELD_SIZE as i32
}

const CARD_UI_ID: UiId = 1;

static mut a: i32 = 0;

//calling this once will swallow multiple clicks on the button. We could either
//pass in and return the number of clicks to fix that, or this could simply be
//called multiple times per frame (once for each click).
fn do_button(platform: &Platform,
             context: &mut UIContext,
             spec: &ButtonSpec,
             id: UiId,
             left_mouse_pressed: bool,
             left_mouse_released: bool)
             -> bool {
    let mut result = false;

    let mouse_pos = (platform.mouse_position)();
    let inside = inside_rect(mouse_pos, spec.x, spec.y, spec.w, spec.h);

    if context.active == id {
        if left_mouse_released {
            result = context.hot == id && inside;

            context.set_not_active();
        }
    } else if context.hot == id {
        if left_mouse_pressed {
            context.set_active(id);
        }
    }

    if inside {
        context.set_next_hot(id);
    }

    if context.active == id && (platform.key_pressed)(KeyCode::MouseLeft) {
        draw_rect_with(platform,
                       spec.x,
                       spec.y,
                       spec.w,
                       spec.h,
                       ["╔", "═", "╕", "║", "│", "╙", "─", "┘"]);
    } else if context.hot == id {
        draw_rect_with(platform,
                       spec.x,
                       spec.y,
                       spec.w,
                       spec.h,
                       ["┌", "─", "╖", "│", "║", "╘", "═", "╝"]);
    } else {
        draw_rect(platform, spec.x, spec.y, spec.w, spec.h);
    }

    let rect_middle = spec.x + (spec.w / 2);

    (platform.print_xy)(rect_middle - (spec.text.len() as i32 / 2),
                        spec.y + (spec.h / 2),
                        &spec.text);

    return result;
}

pub fn over_address(game: &Game, mouse_pos: Point) -> Option<usize> {
    let card_upper_left = mouse_pos.add(CARD_MOUSE_X_OFFSET, CARD_MOUSE_Y_OFFSET);

    if card_upper_left.x > game.playfield_right_edge {
        return None;
    }

    //plus 1 to skip the top edge of the card
    let address = game.scroll_offset + card_upper_left.y + 1;

    if address >= 0 && address < PLAYFIELD_SIZE as i32 {
        Some(address as usize)
    } else {
        None
    }

}

pub fn draw(platform: &Platform, game: &Game) {

    (platform.print_xy)(32,
                        16,
                        &format!("hot : {}, active : {}",
                                 game.ui_context.hot,
                                 game.ui_context.active));

    (platform.print_xy)(32, 0, "load state A\nand A 0b0001\nJZ SLOT1");

    draw_instructions(platform, game);

    let selected = game.selected_card.unwrap_or(std::usize::MAX);

    for i in 0..game.cards.len() {
        let ref card = game.cards[i];

        if i != selected {
            draw_card(platform, card);
        }
    }

    if let Some(card) = game.cards.get(selected) {
        let mouse_pos = (platform.mouse_position)();

        let card_upper_left = mouse_pos.add(CARD_MOUSE_X_OFFSET, CARD_MOUSE_Y_OFFSET);

        draw_card_at(platform, card_upper_left, card);

        if over_address(game, mouse_pos).is_some() {
            (platform.print_xy)(card_upper_left.x, card_upper_left.y + 1, "<");
        }
    }

    draw_registers(platform, game);
}

const CARD_WIDTH: i32 = 16;
const CARD_HEIGHT: i32 = 12;

const CARD_MOUSE_X_OFFSET: i32 = -CARD_WIDTH / 2;
const CARD_MOUSE_Y_OFFSET: i32 = 0;


fn draw_card(platform: &Platform, card: &Card) {
    draw_card_at(platform, card.location, card);
}

fn draw_card_at(platform: &Platform, location: Point, card: &Card) {
    let x = location.x;
    let y = location.y;

    draw_rect(platform, x, y, CARD_WIDTH, CARD_HEIGHT);

    let mut index = 0;
    for i in (y + 1)..(y + CARD_HEIGHT - 1) {
        if let Some(instruction) = card.instructions.get(index) {
            let mut instr_str = format!("{}", instruction);
            instr_str.truncate(CARD_WIDTH as usize - 2);

            (platform.print_xy)(x + 1, i, &instr_str);
        }

        index += 1;
    }
}

fn draw_rect(platform: &Platform, x: i32, y: i32, w: i32, h: i32) {
    draw_rect_with(platform,
                   x,
                   y,
                   w,
                   h,
                   ["┌", "─", "┐", "│", "│", "└", "─", "┘"]);
}

fn draw_double_line_rect(platform: &Platform, x: i32, y: i32, w: i32, h: i32) {
    draw_rect_with(platform,
                   x,
                   y,
                   w,
                   h,
                   ["╔", "═", "╗", "║", "║", "╚", "═", "╝"]);
}

fn draw_rect_with(platform: &Platform, x: i32, y: i32, w: i32, h: i32, edges: [&str; 8]) {
    (platform.clear)(Some(Rect::from_values(x, y, w, h)));

    let right = x + w - 1;
    let bottom = y + h - 1;
    // top
    (platform.print_xy)(x, y, edges[0]);
    for i in (x + 1)..right {
        (platform.print_xy)(i, y, edges[1]);
    }
    (platform.print_xy)(right, y, edges[2]);

    // sides
    for i in (y + 1)..bottom {
        (platform.print_xy)(x, i, edges[3]);
        (platform.print_xy)(right, i, edges[4]);
    }

    //bottom
    (platform.print_xy)(x, bottom, edges[5]);
    for i in (x + 1)..right {
        (platform.print_xy)(i, bottom, edges[6]);
    }
    (platform.print_xy)(right, bottom, edges[7]);
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

#[no_mangle]
pub fn clamp_scroll_offset(height: i32, scroll_offset: i32) -> i32 {
    let len = common::PLAYFIELD_SIZE as i32;

    clamp!(-height + 1, scroll_offset, len - 1)
}

const ALT_FG: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};
const ALT_BG: Color = Color {
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};
const STANDARD_FG: Color = Color {
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};
const STANDARD_BG: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};

const REGISTERS_PER_ROW: i32 = 4;
const REGISTER_DISPLAY_WIDTH: i32 = 8;
const REGISTER_DISPLAY_HEIGHT: i32 = 1;
const REGISTERS_X_OFFSET: i32 = REGISTER_DISPLAY_WIDTH * REGISTERS_PER_ROW;
const REGISTERS_Y_OFFSET: i32 = 0;

fn draw_registers(platform: &Platform, game: &Game) {
    let width = (platform.size)().width - REGISTERS_X_OFFSET;

    for y in 0..((REGISTER_AMOUNT as i32) / REGISTERS_PER_ROW) {
        for x in 0..REGISTERS_PER_ROW {
            let register_number = y * REGISTERS_PER_ROW + x;

            if let Some(register) = common::to_register(register_number) {


                (platform.print_xy)((x * REGISTER_DISPLAY_WIDTH) + width,

                                    (y * REGISTER_DISPLAY_HEIGHT) + REGISTERS_Y_OFFSET,
                                    &format!("{:?}:{:#04X}",
                                             register,
                                             game.registers[register_number as usize]));
            }
        }
    }
}

fn draw_instructions(platform: &Platform, game: &Game) {

    let height = (platform.size)().height;
    let scroll_offset = clamp_scroll_offset(height, game.scroll_offset);

    for y in 0..height {
        let address = y + scroll_offset;
        if let Some(instruction) = game.instructions.get(address as usize) {
            if Some(address) == game.executing_address {
                (platform.set_colors)(ALT_FG, ALT_BG);
                (platform.print_xy)(0, y, format!("{:#04X}│{}", address, instruction).as_ref());
                (platform.set_colors)(STANDARD_FG, STANDARD_BG);
            } else {
                (platform.print_xy)(0, y, format!("{:#04X}│{}", address, instruction).as_ref());
            }
        } else if address == -1 {
            (platform.print_xy)(0, y, "────┐");
        } else if address == common::PLAYFIELD_SIZE as i32 {
            (platform.print_xy)(0, y, "────┘");
        } else {
            //don't print anything
        }
    }
}

pub fn clicked_card(game: &Game, mouse_position: Point) -> Option<usize> {
    //we iterate thisbackwards because we want the top one (the last drawn)
    //and the cards are drawn in forwards order,
    for i in (0..game.cards.len()).rev() {
        let ref card = game.cards[i];

        if inside_rect(mouse_position,
                       card.location.x,
                       card.location.y,
                       CARD_WIDTH,
                       CARD_HEIGHT) {
            return Some(i);
        }
    }

    None
}

pub fn inside_rect(point: Point, x: i32, y: i32, w: i32, h: i32) -> bool {
    x <= point.x && y <= point.y && point.x < x + w && point.y < y + h

}
