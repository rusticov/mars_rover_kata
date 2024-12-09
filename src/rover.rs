use crate::direction::Direction;

pub struct Rover {
    direction: Direction,
}

impl Rover {
    pub fn execute_commands(&mut self, commands: &str) -> String {
        for _command in commands.chars() {
            self.direction = self.direction.turn_right();
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
