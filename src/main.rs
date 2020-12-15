mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use std::collections::HashMap;

fn read_input_data(file_name: &str) -> Vec<i64> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let line = f.lines().next().unwrap().unwrap();

    return line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
}

fn solution(file_name: &str, steps: i64) -> i64 {
    let starting_nums = read_input_data(file_name);
    let len_input = starting_nums.len();
    let mut last_index_dict: HashMap<i64, i64> = HashMap::new();
    let mut last_index: i64;
    for i in 0..len_input-1 {
        last_index_dict.insert(starting_nums[i], i as i64);
    }
    let mut last_num: i64 = starting_nums[len_input-1];
    for i in (len_input as i64-1)..(steps-1) {
        last_index = *last_index_dict.get(&last_num).unwrap_or(&i);
        last_index_dict.insert(last_num, i);
        last_num = i - last_index;
        debug!("{:?}", last_num)
    }
    return last_num;
}

fn main() {
    env_logger::init();
    info!("{:?}", solution("input_data.txt", 2020));
    info!("{:?}", solution("input_data.txt", 30000000));
}
