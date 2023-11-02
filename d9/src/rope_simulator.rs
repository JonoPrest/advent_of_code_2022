#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Motion {
    pub direction: Direction,
    pub amount: i32,
}

impl Motion {
    pub fn new(direction: Direction, amount: i32) -> Self {
        Motion { amount, direction }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    fn init() -> Self {
        Self::new(0, 0)
    }

    fn move_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y + 1),
            Direction::Down => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }

    fn val_is_within_1_range(a: i32, b: i32) -> bool {
        a >= b - 1 && a <= b + 1
    }

    fn is_within_1_range(&self, cmp: &Coordinate) -> bool {
        Self::val_is_within_1_range(self.x, cmp.x) && Self::val_is_within_1_range(self.y, cmp.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct RopeCoordinate {
    pub head: Coordinate,
    pub tail: Coordinate,
}

impl RopeCoordinate {
    fn new(head: Coordinate, tail: Coordinate) -> Self {
        RopeCoordinate { head, tail }
    }

    fn init() -> Self {
        Self::new(Coordinate::init(), Coordinate::init())
    }

    fn move_rope(&self, direction: &Direction) -> Self {
        match (self, direction) {
            (rope_co, direction)
                //Only move the head in the the case
                //where the move destination is within 1
                //range of tail
                if rope_co.head.move_direction(&direction).is_within_1_range(&rope_co.tail) =>
            {
              Self::new(
                    rope_co.head.move_direction(direction),
                    rope_co.tail.clone(),
                )
            }
            //In the case where they are in the same row and moving left or right
            //Or same column moving up or down
            //both head and tail move in that direction
            (rope_co, direction)
                if (rope_co.head.y == rope_co.tail.y
                    && matches!(direction, Direction::Left | Direction::Right))
                ||(rope_co.head.x == rope_co.tail.x
                    && matches!(direction, Direction::Up | Direction::Down) ) =>
            {
                 Self::new(
                    rope_co.head.move_direction(direction),
                    rope_co.tail.move_direction(direction),
                )
            }
            //In the default case where head and tail are not on the same column or row
            //and the head move is not within 1 of the tail, head moves and the tail occupies
            //the space of the head was previously in
            (rope_co, direction) => {
                 Self::new(
                    rope_co.head.move_direction(direction),
                    rope_co.head.clone(),
                )

            }
        }
    }
}

type RopeCoordinates = Vec<RopeCoordinate>;

pub struct RopeMap {
    pub path: RopeCoordinates,
}

impl RopeMap {
    pub fn init() -> Self {
        let init_coordinate = RopeCoordinate::init();
        RopeMap {
            path: vec![init_coordinate],
        }
    }

    fn get_last_co(&self) -> &RopeCoordinate {
        self.path
            .last()
            .expect("Should always be at least 1 coordinate")
    }

    fn move_rope(&mut self, direction: &Direction) {
        let last_co = self.get_last_co();
        let next_co = last_co.move_rope(direction);
        self.path.push(next_co);
    }

    pub fn move_rope_n_times(&mut self, direction: Direction, n: i32) {
        for _ in 0..n {
            self.move_rope(&direction);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::{Down, Left, Right, Up};

    #[test]
    fn validate_within_one_range_path() {
        let co_1 = Coordinate::new(1, 1);
        let co_2 = Coordinate::new(1, 2);
        let co_3 = Coordinate::new(2, 2);
        let co_4 = Coordinate::new(2, 1);
        let co_5 = Coordinate::new(2, 0);
        let co_6 = Coordinate::new(1, 0);
        let co_7 = Coordinate::new(0, 0);
        let co_8 = Coordinate::new(0, 1);
        let co_9 = Coordinate::new(0, 2);
        // let co_4 = Coordinate::new(0, 0);
        // let co_4 = Coordinate::new(0, 1);

        assert!(co_1.is_within_1_range(&co_1));
        assert!(co_1.is_within_1_range(&co_2));
        assert!(co_1.is_within_1_range(&co_3));
        assert!(co_1.is_within_1_range(&co_4));
        assert!(co_1.is_within_1_range(&co_5));
        assert!(co_1.is_within_1_range(&co_6));
        assert!(co_1.is_within_1_range(&co_7));
        assert!(co_1.is_within_1_range(&co_8));
        assert!(co_1.is_within_1_range(&co_9));

        // let bad_co_3 = Coordinate::new(1, 3);
        // let bad_co_6 = Coordinate::new(0, 2);
        // assert!(!co_1.is_within_1_range(&co_3));
        // assert!(!co_1.is_within_1_range(&co_6));
    }

    #[test]
    fn validate_move_case_same_row_right() {
        let head_start = Coordinate::new(2, 1);
        let tail_start = Coordinate::new(1, 1);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let head_end = Coordinate::new(3, 1);
        let tail_end = Coordinate::new(2, 1);
        let co_end = RopeCoordinate::new(head_end, tail_end);

        assert_eq!(co_end, co_start.move_rope(&Right));
    }

    #[test]
    fn validate_move_case_same_col_up() {
        let head_start = Coordinate::new(2, 2);
        let tail_start = Coordinate::new(2, 1);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let head_end = Coordinate::new(2, 3);
        let tail_end = Coordinate::new(2, 2);
        let co_end = RopeCoordinate::new(head_end, tail_end);

        assert_eq!(co_end, co_start.move_rope(&Up));
    }

    #[test]
    fn validate_move_case_diagonal_up() {
        let head_start = Coordinate::new(2, 2);
        let tail_start = Coordinate::new(1, 1);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let head_end = Coordinate::new(2, 3);
        let tail_end = Coordinate::new(2, 2);
        let co_end = RopeCoordinate::new(head_end, tail_end);

        assert_eq!(co_end, co_start.move_rope(&Up));
    }

    #[test]
    fn validate_move_case_diagonal_right() {
        let head_start = Coordinate::new(2, 2);
        let tail_start = Coordinate::new(1, 1);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let head_end = Coordinate::new(3, 2);
        let tail_end = Coordinate::new(2, 2);
        let co_end = RopeCoordinate::new(head_end, tail_end);

        assert_eq!(co_end, co_start.move_rope(&Right));
    }

    #[test]
    fn validate_move_case_same_col_overlap_down() {
        let head_start = Coordinate::new(1, 2);
        let tail_start = Coordinate::new(1, 1);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let head_end = Coordinate::new(1, 1);
        let tail_end = Coordinate::new(1, 1);
        let co_end = RopeCoordinate::new(head_end, tail_end);

        assert_eq!(co_end, co_start.move_rope(&Down));
    }
}
