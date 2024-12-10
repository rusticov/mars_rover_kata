use crate::direction::Direction;
use crate::grid::{Grid, Location};

#[derive(Default)]
pub struct Rover {
    grid: Grid,
    direction: Direction,
    location: Location,
}

impl Rover {
    pub fn execute_commands(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                'M' => {
                    let location = self.location.move_forwards(self.direction);
                    self.location = self.grid.move_to_location(location, self.direction);
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
