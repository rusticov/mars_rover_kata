use crate::direction::Direction;

type Coordinate = i8;

#[derive(Default)]
struct Location {
    x: Coordinate,
    y: Coordinate,
}

impl Location {
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
                    match self.direction {
                        Direction::North => {
                            self.location.y += 1;
                        }
                        Direction::South => self.location.y -= 1,
                        Direction::East => {
                            self.location.x += 1;
                        }
                        Direction::West => self.location.x -= 1,
                    }
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
