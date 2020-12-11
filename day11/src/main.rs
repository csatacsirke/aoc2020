use lib_aoc::Coord;

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Empty, Occupied, Floor, OutOfBounds
}

impl Tile {
    fn parse(char: char) -> Tile {
        match char {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => panic!()
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let char = match self {
            Tile::Empty => "L",
            Tile::Occupied => "#",
            Tile::Floor => ".",
            _ => " "
        };
        write!(f, "{}", char)
    }
}

// impl Eq for Tile {
//     fn eq(&self, other: &Rhs) -> bool {

//     }
// }
 

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    tiles: Vec<Tile>,
    width: i32,
    height: i32
}

impl Grid {
    fn parse(input: &Vec<String>) -> Grid {
        let height = input.len() as i32;
        let width = input.first().unwrap().len() as i32;
        let mut tiles = Vec::<Tile>::new();
        for line in input.iter() {
            tiles.extend(line.chars().map(|char| Tile::parse(char)))
        }

        Grid {
            tiles, width, height
        }
    }

    fn get<T: Into<Coord>>(&self, c: T) -> Tile {
        let c : Coord = c.into();

        if !(0..self.width as i32).contains(&c.x) || !(0..self.height as i32).contains(&c.y) {
            return Tile::OutOfBounds;
        }

        let index = c.y * self.width + c.x;

        return self.tiles[index as usize].clone();
    }

    fn set<T: Into<Coord>>(&mut self, c: T, new_tile: Tile) {
        let c : Coord = c.into();
        
        if !(0..self.width as i32).contains(&c.x) || !(0..self.height as i32).contains(&c.y) {
            panic!();
        }

        let index = c.y * self.width + c.x;

        self.tiles[index as usize] = new_tile;
    }

    
    fn calculate_new_tile_v1<T: Into<Coord>>(&self, center: T) -> Tile {
        let center : Coord = center.into();
        let center_tile = self.get(center);

        if let Tile::Floor = center_tile {
            return Tile::Floor;
        }

        let neighbours: &[(i32, i32)] = &[
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),         (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        let mut occupied_count = 0;
        for &delta in neighbours {
            let c = Coord::from(delta) + center;
            let tile = self.get(c);
            if Tile::Occupied == tile {
                occupied_count += 1;
            }
        }

        if Tile::Empty == center_tile && occupied_count == 0 {
            return Tile::Occupied;
        }

        if Tile::Occupied == center_tile && occupied_count >= 4 {
            return Tile::Empty;
        }

        return center_tile;
    }

    
    fn calculate_new_tile_v2<T: Into<Coord>>(&self, center: T) -> Tile {
        let center : Coord = center.into();
        let center_tile = self.get(center);

        if let Tile::Floor = center_tile {
            return Tile::Floor;
        }

        let neighbours: &[(i32, i32)] = &[
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),         (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        let mut occupied_count = 0;
        for &delta in neighbours {
            let mut c = center;
            loop {
                c += Coord::from(delta);
                let tile = self.get(c);
                if Tile::Occupied == tile {
                    occupied_count += 1;
                }

                if Tile::Floor == tile {
                    continue;
                }

                break;
            }
            
        }

        if Tile::Empty == center_tile && occupied_count == 0 {
            return Tile::Occupied;
        }

        if Tile::Occupied == center_tile && occupied_count >= 5 {
            return Tile::Empty;
        }

        return center_tile;
    }
}


type SeatPredicate = fn(grid: &Grid, center: (i32, i32)) -> Tile;

fn iterate_grid(grid: &Grid, predicate: SeatPredicate) -> Grid {
    let mut new_grid = grid.clone();
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            let new_tile = predicate(grid, (x, y));
            new_grid.set((x, y), new_tile);
        }
    }


    return new_grid;
}

fn print_grid(grid: &Grid) {
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{}", grid.get((x, y)));
        }
        println!();
    }

    println!();
    println!();
    println!();
}

fn part1(input: &Vec<String>) -> String {

    let mut grid = Grid::parse(input);
    loop {
        let new_grid = iterate_grid(&grid, |grid, c| grid.calculate_new_tile_v1(c));

        //print_grid(&new_grid);

        if new_grid == grid {
            break;
        }

        grid = new_grid;
    }

    let occupied_count = grid.tiles
        .iter()
        .filter(|&tile| *tile == Tile::Occupied)
        .count();

    return occupied_count.to_string();
}


fn part2(input: &Vec<String>) -> String {
    
    let mut grid = Grid::parse(input);
    loop {
        let new_grid = iterate_grid(&grid, |grid, c| grid.calculate_new_tile_v2(c));

        //print_grid(&new_grid);

        if new_grid == grid {
            break;
        }

        grid = new_grid;
    }

    let occupied_count = grid.tiles
        .iter()
        .filter(|&tile| *tile == Tile::Occupied)
        .count();

    return occupied_count.to_string();
}

fn main() { 
    lib_aoc::run_with_test("day11", Some(part1), Some(part2));

}

