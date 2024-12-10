use crate::direction::Direction;
use crate::{Grid, Location};

pub struct Rover<'a> {
    grid: &'a Grid,
    direction: Direction,
    location: Location,
}

impl<'a> Rover<'a> {
    const ENCOUNTERED_OBSTABLE_RESULT_PREFIX: &'static str = "O:";

    pub fn new(grid: &'a Grid) -> Self {
        Rover {
            grid,
            direction: Default::default(),
            location: Default::default(),
        }
    }

    pub fn execute_commands(&mut self, commands: &str) -> String {
        let mut result_prefix = "";
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                'M' => {
                    match self
                        .grid
                        .check_rover_move_against_rules(self.location.move_forwards(self.direction))
                    {
                        Some(new_location) => self.location = new_location,
                        None => {
                            result_prefix = Self::ENCOUNTERED_OBSTABLE_RESULT_PREFIX;
                            break;
                        }
                    }
                }
                _ => (),
            }
        }

        format!(
            "{}{}:{}:{}",
            result_prefix,
            self.location.east(),
            self.location.north(),
            self.direction.char()
        )
    }
}
