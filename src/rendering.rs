use crate::board::{Board, IntersectionState};

pub const WHITE_STONE: char = '●';
pub const BLACK_STONE: char = '○';

pub const BOX_TL_CORNER: char = '┌';
pub const BOX_TR_CORNER: char = '┐';
pub const BOX_BL_CORNER: char = '└';
pub const BOX_BR_CORNER: char = '┘';

pub const BOX_LEFT_EDGE: char = '├';
pub const BOX_RIGHT_EDGE: char = '┤';
pub const BOX_TOP_EDGE: char = '┬';
pub const BOX_BOTTOM_EDGE: char = '┴';

pub const BOX_INTERSECTION: char = '┼';

pub const ANSI_BASE: &str = "\x1b[";
pub const ANSI_SET_FG: &str = "38;2;";
pub const ANSI_SET_BG: &str = "42;2;";

pub const ANSI_RESET: &str = "0m";
pub const ANSI_WHITE: &str = "37m";
pub const ANSI_BOLD: &str = "1m";

// Color pallete: https://coolors.co/deab2b-c2941e-f3f7f4-000022
pub const BG_COLOR: &str = "194;171;43m";
pub const LINE_COLOR: &str = "194;148;30m";
pub const WHITE_COLOR: &str = "255;255;255m";
pub const BLACK_COLOR: &str = "0;0;34m";

// Horizontally we insert a line after each character to show a proper square in the terminal
pub const BOX_LINE: char = '─';

/// Renders a board to stdout.
pub fn render_board(board: &Board) {
    let data = board.get_data();

    print!("{}{}{}", ANSI_BASE, ANSI_SET_BG, BG_COLOR);
    print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, LINE_COLOR);

    for x in 0..board.size() {
        let mut num_string = (x + 1).to_string();

        while num_string.len() < 2 {
            num_string.push(' ');
        }

        print!("{}", num_string);
    }
    // Print a space in the end to fill the board top right corner
    print!(" ");

    print!("\n");

    for y in 0..board.size() {
        for x in 0..board.size() {
            let state = data[y][x];

            let mut y_indicator_string = (y + 1).to_string();

            while y_indicator_string.len() < 2 {
                y_indicator_string.push(' ');
            }

            print!("{}{}{}", ANSI_BASE, ANSI_SET_BG, BG_COLOR);

            if state == IntersectionState::Empty {
                print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, LINE_COLOR);
                if y == 0 {
                    if x == 0 {
                        print!("{}", BOX_TL_CORNER);
                    } else if x == board.size() - 1 {
                        print!("{} {}", BOX_TR_CORNER, y_indicator_string);
                    } else {
                        print!("{}", BOX_TOP_EDGE);
                    }
                } else if y == board.size() - 1 {
                    if x == 0 {
                        print!("{}", BOX_BL_CORNER);
                    } else if x == board.size() - 1 {
                        print!("{} {}", BOX_BR_CORNER, y_indicator_string);
                    } else {
                        print!("{}", BOX_BOTTOM_EDGE);
                    }
                } else {
                    if x == 0 {
                        print!("{}", BOX_LEFT_EDGE);
                    } else if x == board.size() - 1 {
                        print!("{} {}", BOX_RIGHT_EDGE, y_indicator_string);
                    } else {
                        print!("{}", BOX_INTERSECTION);
                    }
                }
            } else {
                match state {
                    IntersectionState::Black => {
                        print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, BLACK_COLOR);
                    }
                    IntersectionState::White => {
                        print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, WHITE_COLOR);
                    }
                    IntersectionState::Empty => unreachable!(),
                }
                print!("{}", WHITE_STONE);
                if x == board.size() - 1 {
                    print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, LINE_COLOR);
                    print!(" {}", y_indicator_string);
                }
            }

            if x != board.size() - 1 {
                print!("{}{}{}", ANSI_BASE, ANSI_SET_FG, LINE_COLOR);
                print!("{}", BOX_LINE);
            }
        }
        print!("{}{}", ANSI_BASE, ANSI_RESET);
        print!("\n");
    }
    print!("{}{}", ANSI_BASE, ANSI_RESET);
}
