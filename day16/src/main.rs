use lib_aoc::*;
use std::ops::RangeInclusive;

struct FieldDef {
    field_name: String,
    range1: RangeInclusive<i64>,
    range2: RangeInclusive<i64>,
}

impl FieldDef {
    fn is_valid(&self, value: i64) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

//type Ticket = Vec<i64>;
type PossibilityMatrix = Vec::<Vec::<Option::<bool>>>;


fn check_for_inambiguous_solutions(m: &mut PossibilityMatrix) -> bool {
   

    let field_count = m.len();
    let mut did_change = false;


    let mut new_fields = Vec::<(usize, usize)>::new();

    for ticket_field_index in 0..field_count {
        let mut invalid_count = 0;
        for field_def_index in 0..field_count {
            if m[field_def_index][ticket_field_index] == Some(false) {
                invalid_count += 1;
            }
        }

        
        if invalid_count == field_count - 1 {
            for field_def_index in 0..field_count {
                if m[field_def_index][ticket_field_index] == None {
                    m[field_def_index][ticket_field_index] = Some(true);
                    did_change = true;
                    new_fields.push((field_def_index,ticket_field_index));
                }
            }
        }
    }


    
    for field_def_index in 0..field_count {
        let mut invalid_count = 0;
        for ticket_field_index in 0..field_count {
            if m[field_def_index][ticket_field_index] == Some(false) {
                invalid_count += 1;
            }
        }

        
        if invalid_count == field_count - 1 {
            for ticket_field_index in 0..field_count {
                if m[field_def_index][ticket_field_index] == None {
                    m[field_def_index][ticket_field_index] = Some(true);
                    did_change = true;
                    new_fields.push((field_def_index,ticket_field_index));
                }
            }
        }
    }

    for (new_field_def_index, new_ticket_field_index) in new_fields {
        for field_def_index in 0..field_count {
            if field_def_index != new_field_def_index {
                debug_assert!(m[field_def_index][new_ticket_field_index] != Some(true));
                if m[field_def_index][new_ticket_field_index] == None {
                    m[field_def_index][new_ticket_field_index] = Some(false);
                    did_change = true;
                }
            }
        }

        for ticket_field_index in 0..field_count {
            if ticket_field_index != new_ticket_field_index {
                debug_assert!(m[new_field_def_index][ticket_field_index] != Some(true));
                if m[new_field_def_index][ticket_field_index] == None {
                    m[new_field_def_index][ticket_field_index] = Some(false);
                    did_change = true;
                }
                
            }
        }
    }

    return did_change;
}


fn part1(input: &Vec<String>) -> String {

    let mut input = input.iter();

    let mut field_defs = Vec::<FieldDef>::new();
    
    loop {
        let line = input.next().unwrap();
        if line.len() == 0 {
            break;
        }

        // class: 1-3 or 5-7
        let regex = Regex::new(r"^([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let captures = regex.captures(line).unwrap();

        let _debug = captures[1].to_string();


        field_defs.push(FieldDef {
            field_name: captures[1].to_string(),
            range1: captures[2].parse().unwrap() ..= captures[3].parse().unwrap(),
            range2: captures[4].parse().unwrap() ..= captures[5].parse().unwrap(),
        });
    }

    
    input.next(); // your ticket:
    let _your_ticket = input.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    input.next();
    input.next();

    let mut nearby_tickets = Vec::<Vec<i64>>::new();
    loop {
        let ticket = input.next();
        if let Some(ticket) = ticket {
            let ticket = ticket.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

                
            
            nearby_tickets.push(ticket);
        } else {
            break;
        }

    }

    let mut ticket_scanning_error_rate : i64  = 0;

    for ticket in nearby_tickets {
        for field in ticket {
            let mut is_valid_for_any = false;
            for def in &field_defs {
                if def.is_valid(field) {
                    is_valid_for_any = true; 
                }
            }
            if !is_valid_for_any {
                ticket_scanning_error_rate += field;
            }
            
        }
    }

    return ticket_scanning_error_rate.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut input = input.iter();

    let mut field_defs = Vec::<FieldDef>::new();
    
    loop {
        let line = input.next().unwrap();
        if line.len() == 0 {
            break;
        }

        // class: 1-3 or 5-7
        let regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let captures = regex.captures(line).unwrap();

        field_defs.push(FieldDef {
            field_name: captures[1].to_string(),
            range1: captures[2].parse().unwrap() ..= captures[3].parse().unwrap(),
            range2: captures[4].parse().unwrap() ..= captures[5].parse().unwrap(),
        });
    }

    
    input.next(); // your ticket:
    let your_ticket = input.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    input.next();
    input.next();

    let mut nearby_tickets = Vec::<Vec<i64>>::new();
    loop {
        let ticket = input.next();
        if let Some(ticket) = ticket {
            let ticket = ticket.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

                
            
            nearby_tickets.push(ticket);
        } else {
            break;
        }

    }

    let is_ticket_valid = |ticket: &Vec<i64>| -> bool {
        for field in ticket.iter() {
            let mut is_valid_for_any = false;
            for def in &field_defs {
                if def.is_valid(*field) {
                    is_valid_for_any = true; 
                }
            }
            if !is_valid_for_any {
                return false;
            }
        }
        return true;
    };

    let valid_nearby_tickets: Vec::<Vec<i64>> = nearby_tickets
        .into_iter()
        .filter(|ticket| is_ticket_valid(ticket))
        .collect();

    
    let mut possibility_matrix: PossibilityMatrix =
        (0..field_defs.len())
        .map(|_|  (0..field_defs.len()).map(|_| None).collect())
        .collect();

    //possibility_matrix[def_id][ticket_id];

    debug_assert!(possibility_matrix[0][0] == None);

    for ticket in &valid_nearby_tickets {
        for (ticket_field_index, field) in ticket.iter().enumerate() {
            for (field_def_index, field_def) in field_defs.iter().enumerate() {
                if !field_def.is_valid(*field) {
                    debug_assert!(possibility_matrix[field_def_index][ticket_field_index] != Some(true));
                    possibility_matrix[field_def_index][ticket_field_index] = Some(false);
                }
            }
        }
    }

    loop {
        let did_change = check_for_inambiguous_solutions(&mut possibility_matrix);

        if !did_change {
            break;
        }
    }

    let index_mapping = possibility_matrix.iter()
        .map(|values| values.iter().position(|&x| x == Some(true)).unwrap())
        .collect::<Vec<_>>();

    debug_assert!(index_mapping.len() == field_defs.len());
    
    // mathematical product...
    let mut product = 1;
    for (field_def_index, &ticket_field_index) in index_mapping.iter().enumerate() {
        let field_name = &field_defs[field_def_index].field_name;
        let field_value = your_ticket[ticket_field_index];
        println!("{}: {}", field_name, field_value);

        
        if field_name.starts_with("departure") {
            product *= field_value;
        }

    }

    println!();
    println!();
    println!();


    return product.to_string();
}

fn main() { 

    lib_aoc::run_with_test("day16", Some(part1), Some(part2));

}

