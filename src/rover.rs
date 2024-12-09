use crate::direction::Direction;

pub struct Rover {
    direction: Direction,
}

impl Rover {
    pub fn execute_commands(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => self.direction = self.direction.turn_right(),
                'L' => self.direction = self.direction.turn_left(),
                _ => (),
            }
        }
        format!("0:0:{}", self.direction.char())
    }
}

impl Rover {
    pub fn new() -> Rover {
        Rover {
            direction: Direction::North,
        }
    }
}
