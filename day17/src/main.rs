use lib_aoc::*;
use std::marker::Sized;


type Vec3 = (i64, i64, i64);
type Vec4 = (i64, i64, i64, i64);


trait HasNeighbours : Sized + Clone + Eq + std::hash::Hash {
    fn neighbours(&self) -> Neighbours<Self>;
}

impl<T> HasNeighbours for T where T: HasNeighbours {
    fn neighbours(&self) -> Neighbours<T> {
        Neighbours::<T> {
            center: self.clone(),
            current_index: 0,
            include_self: false,
        }
    }
}

trait HasIndexableNeighbours : Sized {
    fn index_to_relative_neighbour(index: usize) -> Option<Self>;
}

trait VectorLike : HasIndexableNeighbours + HasNeighbours{}


#[derive(Clone)]
struct Neighbours<T> {
    center: T,
    current_index: usize,
    include_self: bool
}

impl<T> Iterator for Neighbours<T> where T: HasIndexableNeighbours {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = T::index_to_relative_neighbour(self.current_index);
        
        delta + self.center
    }
}



fn map_index_to_0_1_minus1(value: usize) -> i64 {
    match value {
        0 => -1,
        1 => 0,
        2 => 1,
        _ => panic!(),
    }
}


impl HasIndexableNeighbours for Vec3 {
    
    fn index_to_relative_neighbour(index: usize) -> Option<Self> {
        if index >= 3*3*3 {
            return None;
        }

        let dx = map_index_to_0_1_minus1(index / (3*3));
        let dy = map_index_to_0_1_minus1((index / 3) % 3);
        let dz = map_index_to_0_1_minus1(index % 3);

        return Some((dx, dy, dz));
    }
}

impl HasIndexableNeighbours for Vec4 {
    
    fn index_to_relative_neighbour(index: usize) -> Option<Self> {
        if index >= 3*3*3 {
            return None;
        }

        let dx = map_index_to_0_1_minus1(index / (3*3*3));
        let dy = map_index_to_0_1_minus1((index / (3*3)) % 3);
        let dz = map_index_to_0_1_minus1((index / 3) % 3);
        let dw = map_index_to_0_1_minus1(index % 3);

        return Some((dx, dy, dz, dw));
    }
}



// impl<T> Iterator for Neighbours<T> {
//     type Item = T;
// }


// impl Iterator for Neighbours<Vec3> {
//     type Item = Vec3;

//     fn next(&mut self) -> Option<Vec3> {
//         if self.current_index >= 3*3*3 {
//             return None;
//         }

//         let (x, y, z) = self.center;

//         let dx = map_index_to_0_1_minus1(self.current_index / (3*3));
//         let dy = map_index_to_0_1_minus1((self.current_index / 3) % 3);
//         let dz = map_index_to_0_1_minus1(self.current_index % 3);

//         self.current_index += 1;

//         if !self.include_self && dx == 0 && dy == 0 && dz == 0 {
//             return self.next();
//         }

//         return Some((x+dx, y+dy, z+dz));
//     }
// }


// impl Iterator for Neighbours<Vec4> {
//     type Item = Vec4;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current_index >= 3*3*3*3 {
//             return None;
//         }

//         let (x, y, z, w) = self.center;

//         let dx = map_index_to_0_1_minus1(self.current_index / (3*3*3));
//         let dy = map_index_to_0_1_minus1((self.current_index / (3*3)) % 3);
//         let dz = map_index_to_0_1_minus1((self.current_index / 3) % 3);
//         let dw = map_index_to_0_1_minus1(self.current_index % 3);

//         self.current_index += 1;

//         if !self.include_self && dx == 0 && dy == 0 && dz == 0 {
//             return self.next();
//         }

//         return Some((x+dx, y+dy, z+dz, w+dw));
//     }
// }

fn collect_possible_points<T>(previous_state: &HashSet<T>) -> HashSet<T> where T: VectorLike {
    let mut new_state = HashSet::<T>::new();

    for point in previous_state {
        for neighbour in point.neighbours() {
            new_state.insert(neighbour);
        }
    }

    return new_state;
}

fn iterate<T>(previous_state: &HashSet<T>) -> HashSet<T> where T: VectorLike {
    let possible_points = collect_possible_points(&previous_state);
    let mut new_state = HashSet::<T>::new();

    for p in possible_points {
        let mut neighbour_count = 0;
        for n in p.neighbours() {
            if previous_state.contains(&n) {
                neighbour_count += 1;
            }
        }

        if previous_state.contains(&p) {
            // If a cube is active and exactly 2 or 3 of its neighbors are also active,
            // the cube remains active. Otherwise, the cube becomes inactive.
            if neighbour_count == 2 || neighbour_count == 3 {
                new_state.insert(p);
            }
        } else {
            // If a cube is inactive but exactly 3 of its neighbors are active,
            //  the cube becomes active. Otherwise, the cube remains inactive.
            if neighbour_count == 3 {
                new_state.insert(p);
            }
        }
    }

    return new_state;
}

fn part1(input: &Vec<String>) -> String {

    let mut map = HashSet::<Vec3>::new();

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                map.insert((x as i64, y as i64, 0));
            }
        }
    }

    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);

    map.len().to_string()
}


fn part2(input: &Vec<String>) -> String {

    let mut map = HashSet::<Vec4>::new();

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                map.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);
    map = iterate(&map);

    map.len().to_string()
}

fn main() { 

    lib_aoc::run_with_test("day17", Some(part1), None);

}

