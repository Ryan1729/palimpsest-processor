extern crate common;

use common::*;
use common::Register::*;
use common::Data::*;
use common::Instruction::*;

#[no_mangle]
pub fn draw(platform: &Platform, game: &Game) {

    (platform.print_xy)(32, 0, "load state A\nand A 0b0001\nJZ SLOT1");

    draw_instructions(platform, game.instructions, game.scroll_offset);

    for card in game.cards.iter() {
        draw_card(platform, card);
    }
}

const CARD_WIDTH: i32 = 16;
const CARD_HEIGHT: i32 = 12;


fn draw_card(platform: &Platform, card: &Card) {
    let x = card.location.x;
    let y = card.location.y;

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
