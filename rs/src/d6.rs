use util;
use std::collections::HashMap;

pub fn run() {
    let input = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp6");

    let split = input.split("\n");
    let vec = split.collect::<Vec<_>>();
    let mut ans_map: HashMap<char, i32> = HashMap::new();

    for i in 0..8 {
        for j in &vec {
            let current_char = j.chars().nth(i).unwrap();
            
            if ans_map.contains_key(&current_char) {
                let count: i32 = *ans_map.get(&current_char).unwrap();
                ans_map.insert(current_char, count + 1);
            } else {
                ans_map.insert(current_char, 0);
            }
        }

        let cal_map = ans_map.clone();
        let cal_map_dup = ans_map.clone();
        
        let mut values: Vec<(&char, &i32)> = cal_map.iter().collect();
        let mut values_dup: Vec<(&char, &i32)> = cal_map_dup.iter().collect();

        values.sort_by(|a, b| a.1.cmp(b.1).reverse());
        values_dup.sort_by(|a, b| a.1.cmp(b.1));
        
        println!("{} {}", values_dup[0].0, values_dup[0].1);
        
        ans_map.clear();
    }
}
