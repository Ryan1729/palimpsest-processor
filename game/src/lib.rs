extern crate common;

use common::*;
use common::Register::*;
use common::Data::*;
use common::Instruction::*;


#[no_mangle]
pub fn new_game(instructions: [Instruction; PLAYFIELD_SIZE], size: Size) -> Game {
    let cards = make_hand(size.height,
                          vec![vec![NOP, NOP, NOP],
                               vec![Load(Value, E), Load(Value, A)],
                               vec![Load(Value, E), NOP],
                               vec![NOP, Load(Value, A)],
                               vec![Load(Value, G), Load(Value, D)]]);

    Game {
        instructions: instructions,
        scroll_offset: 0,
        cards: cards,
        selected_card: Some(0),
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

#[no_mangle]
//returns true if quit requested
pub fn update_and_render(platform: &Platform, game: &mut Game, events: &mut Vec<Event>) -> bool {
    for event in events {

        match *event {
            Event::MouseScroll { delta } => {
                game.scroll_offset = game.scroll_offset.saturating_add(delta);
            }
            Event::KeyPressed { key: KeyCode::MouseLeft, ctrl: _, shift: _ } => {
                game.selected_card = clicked_card(game, (platform.mouse_position)());
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

    draw(platform, game);

    false
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

        draw_card_at(platform, mouse_pos.add(-CARD_WIDTH / 2, 0), card);
    }
}

const CARD_WIDTH: i32 = 16;
const CARD_HEIGHT: i32 = 12;


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
    (platform.clear)(Some(Rect::from_values(x, y, w, h)));

    let right = x + w;
    let bottom = y + h;
    // top
    (platform.print_xy)(x, y, "┌");
    for i in (x + 1)..right {
        (platform.print_xy)(i, y, "─");
    }
    (platform.print_xy)(right, y, "┐");

    // sides
    for i in (y + 1)..bottom {
        (platform.print_xy)(x, i, "│");
        (platform.print_xy)(right, i, "│");
    }

    //bottom
    (platform.print_xy)(x, bottom, "└");
    for i in (x + 1)..right {
        (platform.print_xy)(i, bottom, "─");
    }
    (platform.print_xy)(right, bottom, "┘");
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

        if card.location.x <= mouse_position.x && card.location.y <= mouse_position.y &&
           mouse_position.x < card.location.x + CARD_WIDTH &&
           mouse_position.y < card.location.y + CARD_HEIGHT {
            return Some(i);
        }
    }

    None
}
