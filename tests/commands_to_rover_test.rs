use mars_rover_kata::Rover;
use pretty_assertions::assert_eq;
use yare::parameterized;

#[test]
fn rover_starts_at_origin_facing_north() {
    let mut rover = Rover::new();
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
    let mut rover = Rover::new();

    let rover_end_position = rover.execute_commands(commands);

    pretty_assertions::assert_eq!(rover_end_position, expected_final_rover_position)
}
