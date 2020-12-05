use coord_2d::*;


enum Tile {
    Tree, Empty
}

impl Tile {
    fn from_char(char: char) -> Tile {
        if char == '#' {
            return Tile::Tree;
        }

        return Tile::Empty;
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {

    fn tiles_from_line(line: &String) -> Vec<Tile> {
        line.chars().map(|char| Tile::from_char(char)).collect()
    }

    pub fn new_from_input(input: &Vec<String>) -> Map {
        let tiles = input.iter().map(|line| Self::tiles_from_line(line)).collect();
        return Map {
            tiles
        };
    }

    pub fn get(&self, c: Coord) -> &Tile {
        let row_index = c.y;
        let row = &self.tiles[row_index as usize];
        let tile = &row[(c.x % row.len() as i32) as usize];
        return tile;
    }

    pub fn is_valid_point(&self, c: Coord) -> bool {
        if c.y < 0 || c.y >= self.tiles.len() as i32 {
            return false;
        }

        return true;
    }
}


fn part1(input: &Vec<String>) -> String {
    let map = Map::new_from_input(input);

    let mut tree_count: usize = 0;

    let origin = Coord{ x: 0, y: 0};
    let delta = Coord::new(3, 1);
    for delta in (0..input.len()).map(|y| delta * (y as i32)) {
        let point = origin + delta;
        let tile = map.get(point);
        if let Tile::Tree = tile {
            tree_count += 1;
        }
    }

    return tree_count.to_string();
}


fn part2(input: &Vec<String>) -> String {
    let map = Map::new_from_input(input);


    let deltas = [
        
        Coord::new(1, 1),
        Coord::new(3, 1),
        Coord::new(5, 1),
        Coord::new(7, 1),
        Coord::new(1, 2),
    ];
    let origin = Coord{ x: 0, y: 0};


    let mut answer = 1;
    for delta in deltas.iter() {
        
        let mut tree_count: usize = 0;
        
        for i in 0..input.len() {

            let point = origin + delta * i as i32;
            if !map.is_valid_point(point) {
                break;
            }

            let tile = map.get(point);
            if let Tile::Tree = tile {
                tree_count += 1;
            }
        }

        answer *= tree_count;
    }
    // 6647659200 kicsi volt
    return answer.to_string();
}

fn main() { 
    aoc2020::run_with_test("day03", Some(part1), Some(part2));
}