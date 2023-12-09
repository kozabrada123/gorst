use crate::errors;
use crate::rendering;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
/// Describes possible state of an intersection;
/// It can be empty, filled with a black stone or filled with a white stone.
pub enum IntersectionState {
    #[default]
    Empty,
    Black,
    White,
}

impl fmt::Display for IntersectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "{}", " ")
            }
            Self::Black => {
                write!(f, "{}", rendering::BLACK_STONE)
            }
            Self::White => {
                write!(f, "{}", rendering::WHITE_STONE)
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
/// Describes a go board
pub struct Board {
    /// Data, stored in y, x format;
    /// A vector of rows;
    data: Vec<Vec<IntersectionState>>,
}

impl Board {
    /// Returns the size of the board
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clones and returns inner data.
    pub fn get_data(&self) -> Vec<Vec<IntersectionState>> {
        return self.data.clone();
    }

    /// Overrides the board to the given data
    pub fn set_data(&mut self, data: Vec<Vec<IntersectionState>>) {
        self.data = data;
    }

    /// Creates a new board, size x size big
    pub fn new(size: usize) -> Board {
        let mut board = Board { data: Vec::new() };

        for _y in 0..size {
            let mut row = Vec::new();
            for _x in 0..size {
                row.push(IntersectionState::Empty);
            }
            board.data.push(row);
        }
        return board;
    }

    /// Returns the state of the intersection at x, y
    pub fn get_intersection(&self, x: usize, y: usize) -> Option<IntersectionState> {
        if x >= self.size() || y >= self.size() {
            return None;
        }

        let state = self.data.get(y).unwrap().get(x).unwrap().clone();

        return Some(state);
    }

    /// Sets a state of an intersection, without performing any checks.
    /// Does not adhere to rules.
    pub fn set_intersection(
        &mut self,
        x: usize,
        y: usize,
        state: IntersectionState,
    ) -> Result<(), errors::GoError> {
        if x >= self.size() || y >= self.size() {
            return Err(errors::GoError::InvalidPosition {
                x,
                y,
                size: self.size(),
            });
        }

        self.data[y][x] = state;

        Ok(())
    }

    /// Returns the "direct" liberties of an intersection.
    ///
    /// Direct liberties are liberties above, below, to the right and to the left of the intersection.
    /// (The lines of each intersection axis)
    ///
    /// Each intersection can have between 0 and 4 direct liberties.
    pub fn interesection_direct_liberties(
        &self,
        x: usize,
        y: usize,
    ) -> Result<HashSet<(usize, usize)>, errors::GoError> {
        if x >= self.size() || y >= self.size() {
            return Err(errors::GoError::InvalidPosition {
                x,
                y,
                size: self.size(),
            });
        }

        let mut liberties: HashSet<(usize, usize)> = HashSet::new();

        let possible_relative_liberty_points = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];

        for point in possible_relative_liberty_points {
            // Note: using checked add to account for edges of the board
            // If overflow happens, that isn't a valid position on the board
            let absolute_x_res = x.checked_add_signed(point.0);
            let absolute_y_res = y.checked_add_signed(point.1);

            if absolute_x_res.is_none() || absolute_y_res.is_none() {
                continue;
            }

            let absolute_x = absolute_x_res.unwrap();
            let absolute_y = absolute_y_res.unwrap();

            let state_res = self.get_intersection(absolute_x, absolute_y);

            if let Some(state) = state_res {
                if state == IntersectionState::Empty {
                    liberties.insert((absolute_x, absolute_y));
                };
            }
        }

        return Ok(liberties);
    }

    /// Returns the true liberties of an intersection.
    ///
    /// Stones of the same color share liberties;
    /// This calculates the shared liberties of a connected group of stones.
    ///
    /// Does not work on empty intersections.
    pub fn find_true_liberties(
        &self,
        x: usize,
        y: usize,
    ) -> Result<HashSet<(usize, usize)>, errors::GoError> {
        let home_state_res = self.get_intersection(x, y);

        if let None = home_state_res {
            return Err(errors::GoError::InvalidPosition {
                x,
                y,
                size: self.size(),
            });
        }

        let group: HashSet<(usize, usize)> = self.find_intersections_in_group(x, y).unwrap();

        let mut group_liberties: HashSet<(usize, usize)> = HashSet::new();

        for intersection in group {
            let intersection_liberties = self
                .interesection_direct_liberties(intersection.0, intersection.1)
                .unwrap();
            for liberty in intersection_liberties.into_iter() {
                if !group_liberties.contains(&liberty) {
                    group_liberties.insert(liberty);
                }
            }
        }

        return Ok(group_liberties);
    }

    /// Find all intersections in a group, with (x, y) being the starting stone.
    pub fn find_intersections_in_group(
        &self,
        x: usize,
        y: usize,
    ) -> Result<HashSet<(usize, usize)>, errors::GoError> {
        let home_state_res = self.get_intersection(x, y);

        if let None = home_state_res {
            return Err(errors::GoError::InvalidPosition {
                x,
                y,
                size: self.size(),
            });
        }

        let home_state = home_state_res.unwrap();

        let mut group: HashSet<(usize, usize)> = HashSet::new();
        let mut new_group_stones: HashSet<(usize, usize)> = HashSet::new();
        new_group_stones.insert((x, y));

        while new_group_stones.len() > 0 {
            self.find_stones_in_group_recurse(home_state, &mut group, &mut new_group_stones);
        }

        return Ok(group);
    }

    /// Private function, recursively called to expand the current group
    ///
    /// Finds new intersections connected to the last iteration of new intersections
    fn find_stones_in_group_recurse(
        &self,
        target_state: IntersectionState,
        group: &mut HashSet<(usize, usize)>,
        new_group_stones: &mut HashSet<(usize, usize)>,
    ) {
        let possible_relative_liberty_points = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];

        for intersection in new_group_stones.clone().into_iter() {
            for point in possible_relative_liberty_points.clone() {
                // Note: using checked add to account for edges of the board
                // If overflow happens, that isn't a valid position on the board
                let absolute_x_res = intersection.0.checked_add_signed(point.0);
                let absolute_y_res = intersection.1.checked_add_signed(point.1);

                if absolute_x_res.is_none() || absolute_y_res.is_none() {
                    continue;
                }

                let absolute_x = absolute_x_res.unwrap();
                let absolute_y = absolute_y_res.unwrap();

                if group.contains(&(absolute_x, absolute_y))
                    || new_group_stones.contains(&(absolute_x, absolute_y))
                {
                    continue;
                }

                let state_res = self.get_intersection(absolute_x, absolute_y);

                if let Some(state) = state_res {
                    if state == target_state {
                        new_group_stones.insert((absolute_x, absolute_y));
                    };
                }
            }
            new_group_stones.remove(&intersection);
            group.insert(intersection);
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(9)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        let mut display_board: Vec<Vec<String>> = Vec::new();

        for row in self.data.clone() {
            let mut display_row: Vec<String> = Vec::new();
            for intersection in row {
                display_row.push(format!("{}", intersection))
            }
            display_board.push(display_row);
        }

        for display_row in display_board {
            output.push_str(&format!("{:?}\n", display_row));
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    #[test]
    pub fn direct_liberites() {
        let board = super::Board::new(9);

        let liberties = board.interesection_direct_liberties(4, 4).unwrap();

        assert_eq!(liberties.len(), 4);
        assert_eq!(
            liberties,
            HashSet::from_iter(vec![(4, 3), (3, 4), (5, 4), (4, 5)].iter().cloned())
        );

        let edge_liberties = board.interesection_direct_liberties(0, 0).unwrap();

        assert_eq!(edge_liberties.len(), 2);
        assert_eq!(
            edge_liberties,
            HashSet::from_iter(vec![(0, 1), (1, 0)].iter().cloned())
        );
    }

    #[test]
    pub fn find_group_empty_board() {
        let board = super::Board::new(9);

        let group: HashSet<(usize, usize)> = board.find_intersections_in_group(5, 5).unwrap();

        assert_eq!(group.len(), 81);
    }

    #[test]
    pub fn find_group_1() {
        let mut board = super::Board::new(9);

        board
            .set_intersection(5, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 6, crate::board::IntersectionState::Black)
            .unwrap();

        let group: HashSet<(usize, usize)> = board.find_intersections_in_group(5, 5).unwrap();

        assert_eq!(group.len(), 3);
    }

    #[test]
    pub fn group_liberties_1() {
        let mut board = super::Board::new(9);

        board
            .set_intersection(5, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 6, crate::board::IntersectionState::Black)
            .unwrap();

        let liberties: HashSet<(usize, usize)> = board.find_true_liberties(5, 5).unwrap();

        assert_eq!(liberties.len(), 7);
    }

    #[test]
    pub fn group_liberties_2() {
        let mut board = super::Board::new(9);

        board
            .set_intersection(5, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 6, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(7, 7, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(8, 6, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(4, 5, crate::board::IntersectionState::White)
            .unwrap();

        board
            .set_intersection(5, 4, crate::board::IntersectionState::White)
            .unwrap();

        let liberties: HashSet<(usize, usize)> = board.find_true_liberties(5, 5).unwrap();

        assert_eq!(liberties.len(), 5);
    }

    #[test]
    pub fn group_liberties_3() {
        let mut board = super::Board::new(9);

        board
            .set_intersection(5, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 5, crate::board::IntersectionState::Black)
            .unwrap();
        board
            .set_intersection(6, 6, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(7, 6, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(7, 7, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(8, 6, crate::board::IntersectionState::Black)
            .unwrap();

        board
            .set_intersection(4, 5, crate::board::IntersectionState::White)
            .unwrap();

        board
            .set_intersection(5, 4, crate::board::IntersectionState::White)
            .unwrap();

        let liberties: HashSet<(usize, usize)> = board.find_true_liberties(7, 6).unwrap();

        assert_eq!(liberties.len(), 7);
    }
}
