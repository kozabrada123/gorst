use crate::rendering::render_board;

use board::{Board, IntersectionState};
use inquire;
mod board;
mod errors;
mod rendering;

/// Defines a state in play, with all the necessary data to end the game.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct BoardState {
    pub board: Board,
    /// The number of stones white has captured
    pub white_prisoners: usize,
    /// The number of stones black has captured
    pub black_prisoners: usize,
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            board: Board::default(),
            white_prisoners: 0,
            black_prisoners: 0,
        }
    }

    /// Removes dead groups from the board and updates the counts, returning an updated boardstate
    ///
    /// Optionally you can set last_move, which will process that move last, to have a proper result with ko
    pub fn removed_dead_groups(&self, last_move: Option<(usize, usize)>) -> Self {
        let mut cloned = Self::clone(&self);

        for y in 0..cloned.board.size() {
            for x in 0..cloned.board.size() {
                if let Some(priority) = last_move {
                    if (x, y) == priority {
                        // Check this one last
                        continue;
                    }
                }

                let state = cloned.board.get_intersection(x, y).unwrap();
                if state != IntersectionState::Empty {
                    let group = cloned.board.find_intersections_in_group(x, y).unwrap();
                    let liberties = cloned.board.find_true_liberties(x, y).unwrap();

                    if liberties.len() == 0 {
                        for intersection in group.iter() {
                            cloned
                                .board
                                .set_intersection(
                                    intersection.0,
                                    intersection.1,
                                    IntersectionState::Empty,
                                )
                                .unwrap();

                            match state {
                                IntersectionState::Empty => unreachable!(),
                                IntersectionState::Black => {
                                    cloned.white_prisoners += 1;
                                }
                                IntersectionState::White => {
                                    cloned.black_prisoners += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(priority) = last_move {
            let state = cloned
                .board
                .get_intersection(priority.0, priority.1)
                .unwrap();
            if state != IntersectionState::Empty {
                let group = cloned
                    .board
                    .find_intersections_in_group(priority.0, priority.1)
                    .unwrap();
                let liberties = cloned
                    .board
                    .find_true_liberties(priority.0, priority.1)
                    .unwrap();

                if liberties.len() == 0 {
                    for intersection in group.iter() {
                        cloned
                            .board
                            .set_intersection(
                                intersection.0,
                                intersection.1,
                                IntersectionState::Empty,
                            )
                            .unwrap();

                        match state {
                            IntersectionState::Empty => unreachable!(),
                            IntersectionState::Black => {
                                cloned.white_prisoners += 1;
                            }
                            IntersectionState::White => {
                                cloned.black_prisoners += 1;
                            }
                        }
                    }
                }
            }
        }

        return cloned;
    }
}

fn main() {
    let mut boardstate = BoardState::new();

    // The various states of the board in history, used for undos
    let mut history: Vec<BoardState> = vec![boardstate.clone()];

    loop {
        render_board(&boardstate.board);
        println!("B: {}", boardstate.black_prisoners);
        println!("W: {}", boardstate.white_prisoners);

        let command = inquire::Text::new("").prompt().unwrap();

        let res = parse_move(command, &mut boardstate, &mut history);
        if let Err(e) = res {
            println!("{}", e);
        }
    }
}

/// Parses a move / command and executes it;
///
/// Commands: "exit", "x;y;w/white|b/black|e/erase", "u/undo"
fn parse_move(
    command: String,
    boardstate: &mut BoardState,
    history: &mut Vec<BoardState>,
) -> Result<(), errors::GoError> {
    let lower = command.to_lowercase();

    if lower == "end" {
        std::process::exit(1);
    }

    if lower == "exit" || lower == "quit" {
        std::process::exit(0);
    }

    if lower == "undo" || lower == "u" {
        let history_len = history.len();
        if history_len < 2 {
            return Err(errors::GoError::NothingLeftToUndo);
        }

        // Set the current state to the previous and removes the current state from history
        let previous_boardstate = history[history_len - 2].clone();

        let _ = std::mem::replace(boardstate, previous_boardstate);

        history.remove(history_len - 1);
        return Ok(());
    }

    // i;5;5
    // info;5;5
    // Prints stone liberties
    if lower.contains("i") || lower.contains("info") {
        let params = lower
            .replace(" ", "")
            .split(";")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let x_res = params.get(1).unwrap().parse::<usize>();
        let y_res = params.get(2).unwrap().parse::<usize>();

        if x_res.is_err() || y_res.is_err() {
            return Err(errors::GoError::InvalidMove);
        }

        let x = x_res.unwrap() - 1;
        let y = y_res.unwrap() - 1;

        let direct = boardstate.board.interesection_direct_liberties(x, y);

        if let Err(e) = direct {
            return Err(e);
        }

        let full = boardstate.board.find_true_liberties(x, y);

        if let Err(e) = full {
            return Err(e);
        }

        println!(
            "Direct: {}, True: {}",
            direct.unwrap().len(),
            full.unwrap().len()
        );
        return Ok(());
    }

    if lower.contains(";") {
        let params = lower
            .replace(" ", "")
            .split(";")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // x;y;w|b|e
        // x, y, white / black / empty

        let x_res = params.get(0).unwrap().parse::<usize>();
        let y_res = params.get(1).unwrap().parse::<usize>();

        if x_res.is_err() || y_res.is_err() {
            return Err(errors::GoError::InvalidMove);
        }

        let x = x_res.unwrap() - 1;
        let y = y_res.unwrap() - 1;

        let state = params.get(2);

        if let None = state {
            return Err(errors::GoError::InvalidMove);
        }

        let state = match state.unwrap().as_str() {
            "w" | "white" => IntersectionState::White,
            "b" | "black" => IntersectionState::Black,
            &_ => return Err(errors::GoError::InvalidMove),
        };

        let mut new = boardstate.clone();

        let res = new.board.set_intersection(x, y, state);

        if let Err(e) = res {
            return Err(e);
        }

        new = new.removed_dead_groups(Some((x, y)));

        // A simple ko check is if we are reapeating a board position
        for entry in history.clone() {
            if entry.board == new.board {
                return Err(errors::GoError::KoViolation);
            }
        }

        std::mem::replace(boardstate, new);

        history.push(boardstate.clone());
        return Ok(());
    }

    Ok(())
}
