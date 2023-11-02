#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub x: i32,
    pub y: i32,
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

    fn is_linear_movement(&self, cmp: &Coordinate) -> bool {
        self.x == cmp.x || self.y == cmp.y
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    fn move_head_to_co(&self, next_head_co: Coordinate) -> Self {
        match (self, next_head_co) {
            (rope_co, next_head_co)
                //Only move the head in the the case
                //where the move destination is within 1
                //range of tail
                if next_head_co.is_within_1_range(&rope_co.tail)  =>
            {
              Self::new(
                    next_head_co,
                    rope_co.tail.clone(),
                )
            }
            //In the case where they are in the same row and moving left or right
            //both head and tail move in that direction by the same amount
            (rope_co, next_head_co)
                if !rope_co.head.is_linear_movement(&next_head_co) =>
            {
                 Self::new(next_head_co.clone(),
                        Coordinate::new(
                            rope_co.tail.x + (next_head_co.x - rope_co.head.x),
                            rope_co.tail.y + (next_head_co.y - rope_co.head.y)
                        )
                    )
            }
            //In the default case where head and tail are not on the same column or row
            //and the head move is not within 1 of the tail, head moves and the tail occupies
            //the space of the head was previously in
            (rope_co, next_head_co) => {
                 Self::new(
                    next_head_co,
                    rope_co.head.clone(),
                )

            }
        }
    }

    fn move_rope(&self, direction: &Direction) -> Self {
        let next_head_co = self.head.move_direction(direction);
        self.move_head_to_co(next_head_co)
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

#[derive(Debug)]
pub struct KnottedRope {
    pub knots: RopeCoordinates,
}

impl KnottedRope {
    fn new(knots: RopeCoordinates) -> Self {
        KnottedRope { knots }
    }
    pub fn init(size: usize) -> Self {
        let knots = vec![RopeCoordinate::init(); size];
        Self::new(knots)
    }

    pub fn get_head(&self) -> &RopeCoordinate {
        self.knots.first().expect("Should be a knot at the head")
    }
    pub fn get_tail(&self) -> &RopeCoordinate {
        self.knots.last().expect("Should be a knot at the tail")
    }

    pub fn move_rope(&self, direction: &Direction) -> Self {
        let head_knot = self.get_head();
        let mut next_head_co = head_knot.head.move_direction(direction);
        let mut next_knot_cos = Vec::new();

        for knot in &self.knots {
            let next_knot_co = knot.move_head_to_co(next_head_co);
            next_head_co = next_knot_co.tail.clone();
            next_knot_cos.push(next_knot_co);
        }

        Self::new(next_knot_cos)
    }
}

type KnottedRopes = Vec<KnottedRope>;

#[derive(Debug)]
pub struct KnottedRopeMap {
    pub path: KnottedRopes,
}

impl KnottedRopeMap {
    pub fn init(size: usize) -> Self {
        let init_coordinate = KnottedRope::init(size);
        KnottedRopeMap {
            path: vec![init_coordinate],
        }
    }

    pub fn get_last_co(&self) -> &KnottedRope {
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
    use Direction::{Down, Right, Up};

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

    #[test]
    fn knot_jump_motion() {
        let head_start = Coordinate::new(2, 0);
        let tail_start = Coordinate::new(1, 0);
        let co_start = RopeCoordinate::new(head_start, tail_start);

        let next_co_head = Coordinate::new(3, 1);
        let next_co_tail = Coordinate::new(2, 1);
        let co_end = RopeCoordinate::new(next_co_head.clone(), next_co_tail);

        assert!(!co_start.head.is_linear_movement(&next_co_head));
        assert!(!next_co_head.is_within_1_range(&co_start.tail));
        assert_eq!(co_end, co_start.move_head_to_co(next_co_head));
        //                 && next_head_co.is_within_1_range(&rope_co.tail)
    }
}
