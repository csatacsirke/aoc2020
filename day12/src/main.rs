use lib_aoc::Coord;

fn south() -> Coord {
    (0, -1).into() 
}

fn west() -> Coord {
    (-1, 0).into() 
}

fn north() -> Coord {
    (0, 1).into() 
}

fn east() -> Coord {
    (1, 0).into() 
}



struct Ship {
    position: Coord,
    direction: Coord
}


fn rotate_left(direction: Coord, degrees: i32) -> Coord {
    if degrees == 0 {
        return direction;
    }

    return rotate_left(direction.left90(), degrees - 90);
}

fn rotate_left_around_point(point: Coord, anchor: Coord, degrees: i32) -> Coord {
    let delta = point - anchor;
    let rotated_delta = rotate_left(delta, degrees);
    return anchor + rotated_delta;
}

fn part1(input: &Vec<String>) -> String {
    let mut ship = Ship {
        position: (0, 0).into(),
        direction: east()
    };

    for line in input {
        let regex = regex::Regex::new(r"(\w)(\d+)").unwrap();
        let captures = regex.captures(line).unwrap();
        let key = captures[1].chars().next().unwrap();
        let number: i32 = captures[2].parse().unwrap();
        match key {
            'F' => ship.position += ship.direction * number,

            'N' => ship.position += north() * number,
            'S' => ship.position += south() * number,
            'W' => ship.position += west() * number,
            'E' => ship.position += east() * number,

            // ez valamiért fordítva van, de nincs kedvem kidebugonli hogy miért
            'L' => ship.direction = rotate_left(ship.direction, 360 - number),
            'R' => ship.direction = rotate_left(ship.direction, number),

            _ => panic!()
        }

        //println!("{}, {}", ship.position.x, ship.position.y);
    }

    let answer = ship.position.manhattan_magnitude();
    answer.to_string()
}


fn part2(input: &Vec<String>) -> String {
    let mut ship_position = (0, 0).into();
    let mut waypoint_position = (10, 1).into();

    for line in input {
        let regex = regex::Regex::new(r"(\w)(\d+)").unwrap();
        let captures = regex.captures(line).unwrap();
        let key = captures[1].chars().next().unwrap();
        let number: i32 = captures[2].parse().unwrap();
        match key {
            'F' => {
                let delta = (waypoint_position - ship_position) * number;
                ship_position += delta;
                waypoint_position += delta;
            },

            'N' => waypoint_position += north() * number,
            'S' => waypoint_position += south() * number,
            'W' => waypoint_position += west() * number,
            'E' => waypoint_position += east() * number,

            // ez valamiért fordítva van, de nincs kedvem kidebugonli hogy miért
            'L' => waypoint_position = rotate_left_around_point(waypoint_position, ship_position, 360-number),
            'R' => waypoint_position = rotate_left_around_point(waypoint_position, ship_position, number),

            _ => panic!()
        }

        // println!("ship {}, {}", ship_position.x, ship_position.y);
        // println!("waypoint {}, {}", waypoint_position.x, waypoint_position.y);
    }

    let answer = ship_position.manhattan_magnitude();
    answer.to_string()
}

fn main() { 
    lib_aoc::run_with_test("day12", Some(part1), Some(part2));

}

