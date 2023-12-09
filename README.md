# gorst
Gorst (Go Rust Terminal) is a simple rust app for playing Go in the terminal.

<img src="https://raw.githubusercontent.com/kozabrada123/gorst/main/assets/demo.png" alt="gorst demo" width="512"/>

## Installation

To install, clone the github repo and run `cargo install --path client_local`.

Then you run the app with `grstlc`.

Note: it is recommended to use a terminal which supports runtime font scaling (such as [Alacritty](https://github.com/alacritty/alacritty)), since gorst uses single character rendering, which can look quite small.

By default, the app will use a 9x9 board size.

You can set a custom board size with a command line argument: `grstlc <size>`

For example:

For a 13x13 board: `grstlc 13`

For a 19x19 board: `grstlc 19`

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
| in;{x};{y}     | Prints the liberties of the stone at (x, y) |
| w;{x};{y}      | Places a white stone at (x, y)              |
| b;{x};{y}      | Places a black stone at (x, y)              |
