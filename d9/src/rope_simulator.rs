#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Motion {
    pub direction: Direction,
    pub amount: i32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
}

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

    fn val_is_within_1_range(a: i32, b: i32) -> bool {
        a <= b + 1 && a >= b - 1
    }

    fn co_is_withi_1_range(co_a: &Coordinate, co_b: &Coordinate) -> bool {
        Self::val_is_within_1_range(co_a.x, co_b.x) && Self::val_is_within_1_range(co_a.y, co_b.y)
    }

    fn move_rope(&self, direction: &Direction) -> Self {
        match (self, direction) {
            (rope_co, direction)
                //Only move the head in the following cases 
                //1. they occupy the same coordinate
                if (rope_co.head == rope_co.tail)
                //2. they are on the same row and head is moving up or down
                    || (rope_co.head.y == rope_co.tail.y
                        && matches!(direction, Direction::Up | Direction::Down))
                //3. they are on the same column and moving left or right
                    || (rope_co.head.x == rope_co.tail.x
                        && matches!(direction, Direction::Left | Direction::Right))
                //4. they are not in the same row or column and the head is moving
                //within a range of 1 of the tail
                    ||(rope_co.head.x != rope_co.tail.x
                        &&rope_co.head.y != rope_co.tail.y
                        && matches!(rope_co.head.move_direction(&direction),
                            new_head_coordinate if Self::co_is_withi_1_range(&new_head_coordinate, &rope_co.tail))) =>
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
