// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Self { direction, ..self }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Self { direction, ..self }
    }

    pub fn advance(self) -> Self {
        let (mut x, mut y) = self.position;

        match self.direction {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }

        Self {
            position: (x, y),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {

        let robot = instructions.chars().fold(self, |bot, c| 
            match c {
            'A' => return bot.advance(),
            'L' => return bot.turn_left(),
            'R' => return bot.turn_right(),
            _ => panic!("a"),
            }
        );

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
