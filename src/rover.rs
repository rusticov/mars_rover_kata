use crate::direction::Direction;

type Coordinate = i8;

#[derive(Default)]
struct Location {
    east: Coordinate,
    north: Coordinate,
}

impl Location {
    #[inline]
    fn north(&self) -> Coordinate {
        self.north
    }

    #[inline]
    fn east(&self) -> Coordinate {
        self.east
    }

    fn move_forwards(&self, direction: Direction) -> Location {
        match direction {
            Direction::North => Location {
                east: self.east,
                north: self.north + 1,
            },
            Direction::South => Location {
                east: self.east,
                north: self.north - 1,
            },
            Direction::East => Location {
                east: self.east + 1,
                north: self.north,
            },
            Direction::West => Location {
                east: self.east - 1,
                north: self.north,
            },
        }
    }

    fn teleport_north_south(&self, y: Coordinate) -> Location {
        Location {
            east: self.east,
            north: y,
        }
    }

    fn format(&self, separator: &str) -> String {
        format!("{}{}{}", self.east, separator, self.north)
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
                            if self.location.north() == Self::GRID_LONGITUDE_SIZE {
                                self.location.north = 0;
                            }
                        }
                        Direction::South => {
                            if self.location.north() < 0 {
                                self.location.north = Self::GRID_LONGITUDE_SIZE - 1;
                            }
                        }
                        Direction::East => {
                            if self.location.east() == Self::GRID_LATITUDE_SIZE {
                                self.location.east = 0;
                            }
                        }
                        Direction::West => {
                            if self.location.east() < 0 {
                                self.location.east = Self::GRID_LATITUDE_SIZE - 1;
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
