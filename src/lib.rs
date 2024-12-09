pub struct Rover {
    direction: Direction,
}

impl Rover {
    pub fn execute_commands(&mut self, commands: &str) -> String {
        for _command in commands.chars() {
            self.direction = self.direction.turn_right();
        }
        format!("0:0:{}", self.direction.char())
    }
}

impl Rover {
    pub fn new() -> Rover {
        Rover {
            direction: Direction::North,
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn char(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
