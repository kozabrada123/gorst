use custom_error::custom_error;

custom_error! {
    #[derive(Clone, PartialEq, Eq)]
    pub GoError
    InvalidPosition{x: usize, y: usize, size: usize} = "Position (x: {x}, y: {y}) is invalid for {size}x{size} board",
    InvalidMove = "Couldn't parse move",
    NothingLeftToUndo = "Nothing left to undo",
    KoViolation = "Violation of Ko",
}
