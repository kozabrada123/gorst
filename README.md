# gorst
Gorst (Go Rust Terminal) is a simple rust app for playing Go in the terminal.

## Installation

To install, clone the github repo and run `cargo install --path .`.

Then you run the app with `gorst`.

Note: it is recommended to use a terminal which supports runtime font scaling (such as [Alacritty](https://github.com/alacritty/alacritty)), since gorst uses single character rendering, which can look quite small.

## Usage

After running the app, the starting Go board will be rendered, along with two counters: `B: 0` and `W: 0`.

These two counters show the number of stones (prisoners) each player has taken. (`B` for black prisoners and `W` for white prisoners).

Below the board and counters, there is terminal prompt for commands.

Every interaction with the go board and game is done via text commands, inputed into this prompt.

### Commands

| Command syntax | Effect                                      |
|----------------|---------------------------------------------|
| exit / quit    | Closes the program                          |
| u / undo       | Undos the last move                         |
| i;{x};{y}      | Prints the liberties of the stone at (x, y) |
| {x};{y};w      | Places a white stone at (x, y)              |
| {x};{y};b      | Places a black stone at (x, y)              |
