use crate::board::{Board, IntersectionState};
use crate::gamecommand::GameCommand;

/// Defines a state in play, with all the necessary data to end the game.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BoardState {
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

/// Defines a general state of the game.
///
/// This is the most high level game related struct.
///
/// Holds a history of boardstates, so we can use undo
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct GameState {
    pub history: Vec<BoardState>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            history: vec![BoardState::new()],
        }
    }

    /// Applies a command to the gamestate and returns a copy of self after the command
    pub fn apply_command(&self, command: GameCommand) -> Option<Self> {
        let mut cloned = self.clone();

        match command {
            GameCommand::Set(x, y, state) => {
                let mut boardstate = cloned.history.get(cloned.history.len() - 1)?.clone();
                boardstate.board.set_intersection(x, y, state).ok()?;
                boardstate = boardstate.removed_dead_groups(Some((x, y)));
                cloned.history.push(boardstate);
            }
            GameCommand::Undo => {
                // We can't undo, there is nothing left
                if cloned.history.len() < 2 {
                    return None;
                }

                // Remove the latest boardstate from history, which will make the one before it the latest
                cloned.history.remove(cloned.history.len() - 1);
            }
        }

        return Some(cloned);
    }
}
