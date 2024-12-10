use crate::direction::Direction;

type Coordinate = u8;

pub struct Rover {
    direction: Direction,
    y: Coordinate,
    x: Coordinate,
}

impl Rover {
    const GRID_LONGITUDE_SIZE: Coordinate = 10;
    const GRID_LATITUDE_SIZE: Coordinate = 10;

    pub fn execute_commands(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                'M' => match self.direction {
                    Direction::North => {
                        self.y += 1;
                        if self.y == Self::GRID_LONGITUDE_SIZE {
                            self.y = 0;
                        }
                    }
                    Direction::South => {
                        self.y = if self.y == 0 {
                            Self::GRID_LONGITUDE_SIZE - 1
                        } else {
                            self.y - 1
                        }
                    }
                    Direction::East => {
                        self.x += 1;
                        if self.x == Self::GRID_LATITUDE_SIZE {
                            self.x = 0;
                        }
                    }
                    Direction::West => {
                        self.x = if self.x == 0 {
                            Self::GRID_LATITUDE_SIZE - 1
                        } else {
                            self.x - 1
                        }
                    }
                },
                _ => (),
            }
        }

        format!("{}:{}:{}", self.x, self.y, self.direction.char())
    }
}

impl Rover {
    pub fn new() -> Rover {
        Rover {
            direction: Direction::North,
            y: 0,
            x: 0,
        }
    }
}
