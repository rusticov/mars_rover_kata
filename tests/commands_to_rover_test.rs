use mars_rover_kata::{Grid, Location, Rover};
use pretty_assertions::assert_eq;
use yare::parameterized;

#[test]
fn rover_starts_at_origin_facing_north() {
    let grid = Grid::default();
    let mut rover = Rover::new(&grid);

    let rover_end_position = rover.execute_commands("");

    assert_eq!(rover_end_position, "0:0:N")
}

#[parameterized(
    turn_right_1_time = {"R", "0:0:E"},
    turn_right_2_times = {"RR", "0:0:S"},
    turn_right_3_times = {"RRR", "0:0:W"},
    turn_right_4_times = {"RRRR", "0:0:N"},
    turn_left_1_time = {"L", "0:0:W"},
    turn_left_2_times = {"LL", "0:0:S"},
    turn_left_3_times = {"LLL", "0:0:E"},
    turn_left_4_times = {"LLLL", "0:0:N"},
)]
fn rover_can_turn(commands: &str, expected_final_rover_position: &str) {
    let grid = Grid::default();
    let mut rover = Rover::new(&grid);

    let rover_end_position = rover.execute_commands(commands);

    pretty_assertions::assert_eq!(rover_end_position, expected_final_rover_position)
}

#[parameterized(
    move_north_once = {"M", "0:1:N"},
    move_north_to_end_of_grid = {"MMMMMMMMM", "0:9:N"},
    move_north_to_beyond_end_of_grid = {"MMMMMMMMMM", "0:0:N"},
    move_south_once_wraps_to_top_of_grid = {"LLM", "0:9:S"},
    move_south_multiple_times = {"LLMM", "0:8:S"},
    move_east_once = {"RM", "1:0:E"},
    move_east_to_end_of_grid = {"RMMMMMMMMM", "9:0:E"},
    move_east_to_beyond_end_of_grid = {"RMMMMMMMMMM", "0:0:E"},
    move_west_once_wraps_to_top_of_grid = {"LM", "9:0:W"},
    move_west_multiple_times = {"LMM", "8:0:W"},
)]
fn rover_move_forwards(commands: &str, expected_final_rover_position: &str) {
    let grid = Grid::default();
    let mut rover = Rover::new(&grid);

    let rover_end_position = rover.execute_commands(commands);

    pretty_assertions::assert_eq!(rover_end_position, expected_final_rover_position)
}

#[test]
fn rover_stops_when_obstacle_is_in_the_way() {
    let mut grid = Grid::default();
    grid.add_obstable(Location::new(0, 2));

    let mut rover = Rover::new(&grid);

    let rover_end_position = rover.execute_commands("MM");

    assert_eq!(rover_end_position, "O:0:1:N")
}
