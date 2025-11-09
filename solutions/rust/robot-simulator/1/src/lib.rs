#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, dir: d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.dir = match self.dir {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.dir {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => self, 
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
