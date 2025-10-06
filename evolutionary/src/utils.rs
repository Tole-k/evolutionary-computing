use csv::ReaderBuilder;
use rand::prelude::*;
use std::f64;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct DataPoint {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub cost: i32,
}

pub fn calculate_distance(a: DataPoint, b: DataPoint) -> f64 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f64).sqrt() + b.cost as f64
}

pub fn load_data(path: &str) -> Vec<DataPoint> {
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(path);
    let mut records_mut: Vec<DataPoint> = vec![];

    for (id, record) in reader.unwrap().records().enumerate() {
        let uwrapped_record = record.unwrap();
        let x: i32 = FromStr::from_str(uwrapped_record.get(0).unwrap()).unwrap();
        let y: i32 = FromStr::from_str(uwrapped_record.get(1).unwrap()).unwrap();
        let cost: i32 = FromStr::from_str(uwrapped_record.get(2).unwrap()).unwrap();
        records_mut.push(DataPoint { id, x, y, cost });
    }
    records_mut
}

pub fn check_solution(solution: Vec<usize>, data: Vec<DataPoint>) -> f64 {
    let mut total_value = 0.0;
    let first_point = data[solution[0]];
    let mut last_point = first_point;
    for index in 1..solution.len() {
        let current_point = data[solution[index]];
        total_value += calculate_distance(last_point, current_point);
        last_point = current_point;
    }
    total_value += calculate_distance(last_point, first_point);
    total_value
}

pub fn generate_random_solution(size: usize) -> Vec<usize> {
    let mut nums: Vec<usize> = (0..size).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);
    let half_nums = &nums.clone()[..size / 2];
    half_nums.to_vec()
}
