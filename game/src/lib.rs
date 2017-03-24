extern crate common;

use common::*;
use common::Register::*;
use common::Data::*;
use common::Instruction::*;

#[no_mangle]
pub fn new_game(instructions: [Instruction; PLAYFIELD_SIZE], size: Size) -> Game {
    let cards = make_hand(size.height,
                          vec![vec![NOP, NOP, NOP],
                               vec![Load(Immeadiate(42), E), Load(Immeadiate(42), A)],
                               vec![Load(Immeadiate(42), E), NOP],
                               vec![NOP, Load(Immeadiate(42), A)],
                               vec![Load(Immeadiate(42), G), Load(Immeadiate(42), D)]]);

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
    }
}

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

    (platform.print_xy)(32,
                        16,
                        &format!("hot : {}, active : {}",
                                 game.ui_context.hot,
                                 game.ui_context.active));


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
        unsafe {
            println!("a: {}", a);
            a += 1;
        }
    }

    let test_spec = ButtonSpec {
        x: game.run_button_spec.x,
        y: 8,
        w: game.run_button_spec.w,
        h: 3,
        text: "Test".to_string(),
    };

    if do_button(platform,
                 &mut game.ui_context,
                 &test_spec,
                 -804788405,
                 left_mouse_pressed,
                 left_mouse_released) {
        unsafe {
            println!("a: {}", a);
            a += 1;
        }
    }

    draw(platform, game);

    false
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

    (platform.print_xy)(32, 0, "load state A\nand A 0b0001\nJZ SLOT1");

    draw_instructions(platform, game.instructions, game.scroll_offset);

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

fn draw_instructions(platform: &Platform,
                     instructions: [Instruction; common::PLAYFIELD_SIZE],
                     mut scroll_offset: i32) {

    let height = (platform.size)().height;
    scroll_offset = clamp_scroll_offset(height, scroll_offset);

    for y in 0..height {
        let address = y + scroll_offset;
        if let Some(instruction) = instructions.get(address as usize) {
            (platform.print_xy)(0, y, format!("{:#04X}│{}", address, instruction).as_ref());

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
