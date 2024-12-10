use crate::direction::Direction;

pub type Coordinate = i8;

#[derive(Default)]
pub struct Location {
    east: Coordinate,
    north: Coordinate,
}

impl Location {
    #[inline]
    pub fn north(&self) -> Coordinate {
        self.north
    }

    #[inline]
    pub fn east(&self) -> Coordinate {
        self.east
    }

    fn teleport_north(&self, north: Coordinate) -> Location {
        Location {
            east: self.east,
            north,
        }
    }

    fn teleport_east(&self, east: Coordinate) -> Location {
        Location {
            east,
            north: self.north,
        }
    }

    pub fn move_forwards(&self, direction: Direction) -> Location {
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
}

#[derive(Default)]
pub struct Grid {}

impl Grid {
    const GRID_LONGITUDE_SIZE: Coordinate = 10;
    const GRID_LATITUDE_SIZE: Coordinate = 10;

    pub(crate) fn move_to_location(&self, location: Location, direction: Direction) -> Location {
        match direction {
            Direction::North => {
                if location.north() == Self::GRID_LONGITUDE_SIZE {
                    location.teleport_north(0)
                } else {
                    location
                }
            }
            Direction::South => {
                if location.north() < 0 {
                    location.teleport_north(Self::GRID_LONGITUDE_SIZE - 1)
                } else {
                    location
                }
            }
            Direction::East => {
                if location.east() == Self::GRID_LATITUDE_SIZE {
                    location.teleport_east(0)
                } else {
                    location
                }
            }
            Direction::West => {
                if location.east() < 0 {
                    location.teleport_east(Self::GRID_LATITUDE_SIZE - 1)
                } else {
                    location
                }
            }
        }
    }
}
