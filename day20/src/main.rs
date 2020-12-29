use std::{hash::Hash, unimplemented, vec};

use lib_aoc::*;

type TileId = usize;

trait NormalizeEdge {
    fn normalize_edge(&self) -> Self;
}

impl NormalizeEdge for String {
    fn normalize_edge(&self) -> Self {
        let reversed: String = self.chars().rev().collect();
        let normal = self.clone();
        
        return match normal < reversed {
            true => normal,
            false => reversed,
        };
    }
}

impl NormalizeEdge for Edge {
    fn normalize_edge(&self) -> Self {
        let reversed: Self = self.iter().cloned().rev().collect::<Vec<bool>>();
        let normal = self.clone();
        
        return match normal < reversed {
            true => normal,
            false => reversed,
        };
    }
}


fn add_edge<T>(edge_to_tile_mapping: &mut HashMap<T, HashSet<TileId>>, edge: &T, tile_id: TileId) 
    where T: Hash + Eq + Clone
{

    if let Some(tiles) = edge_to_tile_mapping.get_mut(edge) {
        tiles.insert(tile_id);
    } else {
        edge_to_tile_mapping.insert(edge.clone(), [tile_id].iter().cloned().collect());
    }
}

fn part1(input: &Vec<String>) -> String {

    let re = Regex::new(r"Tile (\d+):").unwrap();

    let mut it = input.iter();
    // let mut edges = Vec::<(String, TileId)>::new();
    let mut edge_to_tile_mapping = HashMap::<String, HashSet<TileId>>::new();

    loop {
        let header = match it.next() {
            Some(header) => header,
            None => break,
        };

        let captures = re.captures(&header).unwrap();
        let tile_id: usize = captures[1].parse().unwrap();

        let mut data = Vec::<String>::new();
        loop {
            let line = match it.next() {
                Some(line) => line,
                None => break,
            };

            if line.len() == 0 {
                break;
            }

            data.push(line.clone());
        }

        
        let left_side: String = data
            .iter()
            .map(|line| line.chars().next().unwrap())
            .collect();
        let right_side: String = data
            .iter()
            .map(|line| line.chars().last().unwrap())
            .collect();

            
        add_edge(&mut edge_to_tile_mapping, &data[0].normalize_edge(), tile_id);
        add_edge(&mut edge_to_tile_mapping, &data[data.len()-1].normalize_edge(), tile_id);

        add_edge(&mut edge_to_tile_mapping, &left_side.normalize_edge(), tile_id);
        add_edge(&mut edge_to_tile_mapping, &right_side.normalize_edge(), tile_id);
    }


    let mut tile_id_to_edge_count_mapping = HashMap::<TileId, usize>::new();
    for (edge, tile_ids) in &edge_to_tile_mapping {
        for tile_id in tile_ids {
            let &previous_edge_count = tile_id_to_edge_count_mapping.get(tile_id).unwrap_or(&0);

            debug_assert!(edge_to_tile_mapping[edge].len() > 0);
            let additional_edge_count = edge_to_tile_mapping[edge].len() - 1;

            tile_id_to_edge_count_mapping.insert(*tile_id, previous_edge_count + additional_edge_count);
        }
    }

    let mut answer = 1;
    for (&tile_id, &count) in &tile_id_to_edge_count_mapping {
        if count == 2 {
            answer *= tile_id;
        }
    }

    answer.to_string()
}


fn part2(input: &Vec<String>) -> String {
    
    let re = Regex::new(r"Tile (\d+):").unwrap();

    let mut it = input.iter();
    
    let mut edge_to_tile_mapping = HashMap::<Edge, HashSet<TileId>>::new();

    let mut tile_datas = HashMap::<TileId, Vec<Vec<Pixel>>>::new();

    loop {
        let header = match it.next() {
            Some(header) => header,
            None => break,
        };

        let captures = re.captures(&header).unwrap();
        let tile_id: usize = captures[1].parse().unwrap();

        let mut data = PixelData::new();
        loop {
            let line = match it.next() {
                Some(line) => line,
                None => break,
            };

            if line.len() == 0 {
                break;
            }

            data.push(to_bool_vec(line));
        }

        // let tile_data: Vec<Vec<bool>> = data
        //     .iter()
        //     .map(|s| to_bool_vec(s))
        //     .collect();
            
        
        let left_side: Edge = data
            .iter()
            .map(|line| *line.iter().next().unwrap())
            .collect();
        let right_side: Edge = data
            .iter()
            .map(|line| *line.iter().last().unwrap())
            .collect();

            
        add_edge(&mut edge_to_tile_mapping, &data[0].normalize_edge(), tile_id);
        add_edge(&mut edge_to_tile_mapping, &data[data.len()-1].normalize_edge(), tile_id);

        add_edge(&mut edge_to_tile_mapping, &left_side.normalize_edge(), tile_id);
        add_edge(&mut edge_to_tile_mapping, &right_side.normalize_edge(), tile_id);

        
        tile_datas.insert(tile_id, data);
    }


    let mut tile_id_to_edge_count_mapping = HashMap::<TileId, usize>::new();
    for (edge, tile_ids) in &edge_to_tile_mapping {
        for tile_id in tile_ids {
            let &previous_edge_count = tile_id_to_edge_count_mapping.get(tile_id).unwrap_or(&0);

            debug_assert!(edge_to_tile_mapping[edge].len() > 0);
            let additional_edge_count = edge_to_tile_mapping[edge].len() - 1;

            tile_id_to_edge_count_mapping.insert(*tile_id, previous_edge_count + additional_edge_count);
        }
    }

    
    let tile_count_in_one_direction = (tile_datas.len() as f64).sqrt().round() as usize;
    let tile_pixel_width = tile_datas.values().next().unwrap().len();
    let picture_width = tile_count_in_one_direction * (tile_pixel_width - 2);

    let mut picture: PixelData =
        (0..picture_width)
            .map(|_| vec![false; picture_width])
            .collect();



    let corner_tile_id = tile_id_to_edge_count_mapping
        .iter()
        .find(|(&_tile_id, &count)| count == 2)
        .and_then(|(&tile_id, &_count)| Some(tile_id))
        .unwrap();

    let mut corner_tile_data = tile_datas[&corner_tile_id].clone();
    
    for i in 0..8 {
        let left_side: Vec<Pixel> = corner_tile_data
            .iter()
            .map(|line| *line.iter().next().unwrap())
            .collect();
        let top_side:Vec<Pixel> = corner_tile_data[0].clone();

        let is_map_edge_left = edge_to_tile_mapping.get(&left_side.normalize_edge()).unwrap().len() == 1;
        let is_map_edge_top = edge_to_tile_mapping.get(&top_side.normalize_edge()).unwrap().len() == 1;

        if is_map_edge_left && is_map_edge_top {
            break;
        }

        if i == 4 {
            corner_tile_data = flip(&corner_tile_data);
        } else {
            corner_tile_data = rotate(&corner_tile_data);
        }
        debug_assert!(i != 7);
    }


    let mut position_to_tile_id_mapping = HashMap::<(usize, usize), TileId>::new();

    position_to_tile_id_mapping.insert((0,0), corner_tile_id);


    paint(&mut picture, (0, 0), &corner_tile_data);
    for i in 0..tile_count_in_one_direction+1 {
        let &prev_id = position_to_tile_id_mapping.get(&(0, i)).unwrap();
        let tile_data = &tile_datas[&prev_id];
        let right_edge = get_right_edge(&tile_data);
        let &next_tile_id = 
            edge_to_tile_mapping[&right_edge.normalize_edge()]
                .iter()
                .find(|&&id| id != prev_id)
                .unwrap();


        let next_tile_data = &tile_datas[&next_tile_id];
        let next_tile_data = rotate_until(next_tile_data, |data| {
            let left_edge = get_left_edge(data);
            return left_edge == right_edge;
        });

        tile_datas.insert(next_tile_id, next_tile_data);
        
        position_to_tile_id_mapping.insert((0, i+1), next_tile_id);
    }

    panic!();
}

fn rotate_until<P>(data: &PixelData, pred: P) -> PixelData
    where P: Fn(&PixelData) -> bool
{
    let mut data = data.clone();

    for i in 0..8 {
        
        if pred(&data) {
            break;
        }

        if i == 4 {
            data = flip(&data);
        } else {
            data = rotate(&data);
        }
        debug_assert!(i != 7);
    }

    return data;
}

fn get_left_edge(data: &PixelData) -> Edge {
    
    data
        .iter()
        .map(|line| *line.iter().next().unwrap())
        .collect()
}

fn get_right_edge(data: &PixelData) -> Edge {
     
    data
        .iter()
        .map(|line| *line.iter().last().unwrap())
        .collect()
}

fn get_top_edge(data: &PixelData) -> Edge {
    data[0].clone()
}

fn get_bottom_edge(data: &PixelData) -> Edge {
    data[data.len()-1].clone()
}

type Edge = Vec<Pixel>;
type Pixel = bool;
type PixelData = Vec<Vec<Pixel>>;


fn to_bool_vec(s: &String) -> Vec::<bool> {
    s.chars()
        .map(|c| match c {
            '.' => false,
            _ => true,
        })
        .collect()
}


fn paint(picture_data: &mut PixelData, (tile_i, tile_j): (usize, usize), tile_data: &PixelData) {
    let tile_width = tile_data.len();
    let offset_x = tile_width * tile_i;
    let offset_y = tile_width * tile_j;

    for j in 0..tile_width {
        for i in 0..tile_width {
            picture_data[offset_y + j][offset_x + i] =
                tile_data[j][i]; 
        }
    }
}

fn rotate(data: &PixelData) -> PixelData {
    let mut rotated = data.clone();
    let len = data.len();

    for j in 0..len {
        for i in 0..len {
            rotated[j][i] = data[i][len - j - 1];
        }
    }

    return rotated;
}

fn flip(data: &PixelData) -> PixelData {
    let mut flipped = data.clone();
    let len = data.len();

    for j in 0..len {
        for i in 0..len {
            flipped[j][i] = data[j][len - i - 1];
        }
    }

    return flipped;
}

fn main() { 

    lib_aoc::run_with_test("day20", Some(part1), Some(part2));

}

