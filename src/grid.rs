use crate::direction::Direction;
use std::cmp::PartialEq;

pub type Coordinate = i8;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Location {
    east: Coordinate,
    north: Coordinate,
}

impl Location {
    pub fn new(east: Coordinate, north: Coordinate) -> Location {
        Location { east, north }
    }

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
pub struct Grid {
    obstacles: Vec<Location>,
}

impl Grid {
    pub fn add_obstable(&mut self, at: Location) {
        self.obstacles.push(at);
    }
}

impl Grid {
    const GRID_SIZE: Coordinate = 10;

    pub(crate) fn check_rover_move_against_rules(&self, to: Location) -> Option<Location> {
        if self.obstacles.iter().any(|x| *x == to) {
            return None;
        }

        let on_grid_location = self.replace_leaving_rover_back_onto_grid(to);

        Some(on_grid_location)
    }

    fn replace_leaving_rover_back_onto_grid(&self, to: Location) -> Location {
        let mut new_location = to;
        if new_location.north() == Self::GRID_SIZE {
            new_location = new_location.teleport_north(0)
        }

        if new_location.north() < 0 {
            new_location = new_location.teleport_north(Self::GRID_SIZE - 1)
        }

        if new_location.east() == Self::GRID_SIZE {
            new_location = new_location.teleport_east(0)
        }

        if new_location.east() < 0 {
            new_location = new_location.teleport_east(Self::GRID_SIZE - 1)
        }

        new_location
    }
}
