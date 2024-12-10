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

    pub(crate) fn move_to_location(&self, location: Location) -> Location {
        let mut new_location = location;
        if new_location.north() == Self::GRID_LONGITUDE_SIZE {
            new_location = new_location.teleport_north(0)
        }

        if new_location.north() < 0 {
            new_location = new_location.teleport_north(Self::GRID_LONGITUDE_SIZE - 1)
        }

        if new_location.east() == Self::GRID_LATITUDE_SIZE {
            new_location = new_location.teleport_east(0)
        }

        if new_location.east() < 0 {
            new_location = new_location.teleport_east(Self::GRID_LATITUDE_SIZE - 1)
        }

        new_location
    }
}
