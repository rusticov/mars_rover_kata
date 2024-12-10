use crate::direction::Direction;
use crate::{Grid, Location};

pub struct Rover<'a> {
    grid: &'a Grid,
    direction: Direction,
    location: Location,
}

impl<'a> Rover<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Rover {
            grid,
            direction: Default::default(),
            location: Default::default(),
        }
    }

    pub fn execute_commands(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                'M' => {
                    let location = self.location.move_forwards(self.direction);
                    self.location = self.grid.move_to_location(location);
                }
                _ => (),
            }
        }

        format!(
            "{}:{}:{}",
            self.location.east(),
            self.location.north(),
            self.direction.char()
        )
    }
}
