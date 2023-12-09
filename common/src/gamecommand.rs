use crate::board::IntersectionState;

/// Defines a command to alter the gamestate
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameCommand {
    /// Sets an intersection at (0, 1) to a state (2)
    Set(usize, usize, IntersectionState),
    /// Undo the previous move
    Undo,
}

impl GameCommand {
    /// Tries to parse a string into a gamecommand
    pub fn try_from_string(input: String) -> Option<GameCommand> {
        let as_lowercase = input.to_lowercase();

        if as_lowercase == "undo" || as_lowercase == "u" {
            return Some(Self::Undo);
        }

        // Set
        // Should be in a format of w;A;5 or b;b;6
        if as_lowercase.starts_with("w") || as_lowercase.starts_with("b") {
            let params = as_lowercase
                .replace(" ", "")
                .split(";")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let state = params.get(0)?;
            let state = match state.as_str() {
                "w" | "white" => IntersectionState::White,
                "b" | "black" => IntersectionState::Black,
                &_ => return None,
            };

            let x_as_ascii = params.get(1)?.to_uppercase().chars().nth(0)?;
            let mut x_result: Option<usize> = None;

            let mut ascii_chars = crate::ASCII.chars();
            for i in 0..crate::ASCII.len() {
                if ascii_chars.next()? == x_as_ascii {
                    x_result = Some(i);
                    break;
                }
            }

            let y_result = params.get(2)?.parse::<usize>().ok();

            let x = x_result?;
            // Y is rendered with a + 1 so we don't start from 0
            let y = y_result? - 1;

            return Some(Self::Set(x, y, state));
        }

        return None;
    }
}
