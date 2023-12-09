use gorst_common::{
    board::Board,
    errors,
    gamecommand::GameCommand,
    gamestate::{BoardState, GameState},
    rendering::render_board,
    ASCII,
};

extern crate gorst_common;

fn main() {
    let mut board_size = 9;

    // Optionally provide the board size as the first argument
    if let Some(size_str) = std::env::args().nth(1) {
        if let Ok(size_as_num) = size_str.parse::<usize>() {
            board_size = size_as_num;
        }
    }

    let mut gamestate = GameState {
        history: vec![BoardState {
            board: Board::new(board_size),
            ..Default::default()
        }],
    };

    loop {
        let latest_boardstate = gamestate.history.get(gamestate.history.len() - 1).unwrap();
        render_board(&latest_boardstate.board);
        println!("B: {}", latest_boardstate.black_prisoners);
        println!("W: {}", latest_boardstate.white_prisoners);

        let command = inquire::Text::new("").prompt().unwrap();

        let res = parse_command(command, &mut gamestate);
        if let Err(e) = res {
            println!("{}", e);
        }
    }
}

/// Parses a command and executes it;
///
/// Commands: "exit", "w/white|b/black;x;y", "u/undo"
fn parse_command(input: String, gamestate: &mut GameState) -> Result<(), errors::GoError> {
    let lower = input.to_lowercase();

    if lower == "end" {
        std::process::exit(1);
    }

    if lower == "exit" || lower == "quit" {
        std::process::exit(0);
    }

    // in;5;5
    // info;5;5
    // Prints stone liberties
    if lower.contains("in") || lower.contains("info") {
        let params = lower
            .replace(" ", "")
            .split(";")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let x_as_ascii = params
            .get(1)
            .unwrap()
            .to_uppercase()
            .chars()
            .nth(0)
            .unwrap();

        let mut asci_chars = ASCII.chars();

        let mut x_res: Option<usize> = None;

        for i in 0..ASCII.len() {
            if asci_chars.next().unwrap() == x_as_ascii {
                x_res = Some(i);
                break;
            }
        }

        let y_res = params.get(2).unwrap().parse::<usize>();

        if x_res.is_none() || y_res.is_err() {
            return Err(errors::GoError::InvalidMove);
        }

        let x = x_res.unwrap();
        let y = y_res.unwrap() - 1;

        let latest_state = gamestate.history.get(gamestate.history.len() - 1).unwrap();

        let direct = latest_state.board.interesection_direct_liberties(x, y);

        if let Err(e) = direct {
            return Err(e);
        }

        let full = latest_state.board.find_true_liberties(x, y);

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

    // Commands that mutate the gamestate, as they have their own parsing
    if let Some(command) = GameCommand::try_from_string(input) {
        let gamestate_result = gamestate.apply_command(command);

        if gamestate_result.is_none() {
            return Err(errors::GoError::InvalidMove);
        }

        let new_gamestate = gamestate_result.unwrap();
        let _ = std::mem::replace(gamestate, new_gamestate);
    }

    Ok(())
}
