use crate::direction::Direction;

type Coordinate = i8;

#[derive(Default)]
struct Location {
    x: Coordinate,
    y: Coordinate,
}

impl Location {
    fn move_forwards(&self, direction: Direction) -> Location {
        match direction {
            Direction::North => Location {
                x: self.x,
                y: self.y + 1,
            },
            Direction::South => Location {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Location {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Location {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn format(&self, separator: &str) -> String {
        format!("{}{}{}", self.x, separator, self.y)
    }
}

#[derive(Default)]
pub struct Rover {
    direction: Direction,
    location: Location,
}

impl Rover {
    const GRID_LONGITUDE_SIZE: Coordinate = 10;
    const GRID_LATITUDE_SIZE: Coordinate = 10;

    pub fn execute_commands(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                'M' => {
                    self.location = self.location.move_forwards(self.direction);
                    match self.direction {
                        Direction::North => {
                            if self.location.y == Self::GRID_LONGITUDE_SIZE {
                                self.location.y = 0;
                            }
                        }
                        Direction::South => {
                            if self.location.y < 0 {
                                self.location.y = Self::GRID_LONGITUDE_SIZE - 1;
                            }
                        }
                        Direction::East => {
                            if self.location.x == Self::GRID_LATITUDE_SIZE {
                                self.location.x = 0;
                            }
                        }
                        Direction::West => {
                            if self.location.x < 0 {
                                self.location.x = Self::GRID_LATITUDE_SIZE - 1;
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        format!("{}:{}", self.location.format(":"), self.direction.char())
    }
}
