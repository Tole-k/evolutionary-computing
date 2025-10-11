use core::f64;
use csv::ReaderBuilder;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use ndarray::{Array1, Array2, Axis};
use std::time::Instant;
use serde_json::Value;

#[derive(Copy, Clone)]
pub struct DataPoint {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub cost: i32,
}

pub struct Metrics {
    pub name: String,
    pub scores: Vec<f64>,
    pub total_time: f64,
    pub best_solution: Vec<usize>,
}

pub fn calculate_distance_matrix(records: &Vec<DataPoint>) -> Array2<f64> {
    let records = Array1::from_vec(records.clone());
    let x = records.map(|s| s.x as f64);
    let y = records.map(|s| s.y as f64);
    let cost = records.map(|s| s.cost as f64);
    let a_x = &x.clone().insert_axis(Axis(1));
    let b_x = &x.insert_axis(Axis(0));
    let a_y = &y.clone().insert_axis(Axis(1));
    let b_y = &y.insert_axis(Axis(0));
    ((a_x - b_x).pow2() + (a_y - b_y).pow2()).sqrt() + cost
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

pub fn check_solution(solution: &Vec<usize>, data: &Vec<DataPoint>, distance_matrix: &Array2<f64>) -> f64 {
    let mut total_value = 0.0;
    let first_point = data[solution[0]];
    let mut last_point = first_point;
    for index in 1..solution.len() {
        let current_point = data[solution[index]];
        total_value += distance_matrix[[last_point.id, current_point.id]];
        last_point = current_point;
    }
    total_value += distance_matrix[[last_point.id, first_point.id]];
    total_value
}

pub fn generate_random_solution(
    data: &Vec<DataPoint>,
    _starting_point_index: usize,
    _distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let size = data.len();
    let mut nums: Vec<usize> = (0..size).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);
    let half_nums = &nums[..size / 2];
    half_nums.to_vec()
}

pub fn benchmark_function(
    f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    name: &str
) -> Metrics {
    let mut scores: Vec<f64> = vec![];
    let mut best_solution_score: f64 = f64::INFINITY;
    let mut best_solution: Vec<usize> = vec![];

    let mut total_time= 0.0;
    for i in 0..data.len() {
        let start_time = Instant::now();
        let solution = f(data, i, distance_matrix);
        total_time += start_time.elapsed().as_secs_f64();
        let solution_score = check_solution(&solution, data, distance_matrix);
        scores.push(solution_score);
        if solution_score < best_solution_score {
            best_solution_score = solution_score;
            best_solution = solution;
        }   
    }
    let name = name.to_string();
    Metrics {
        name,
        scores,
        total_time,
        best_solution,
    }
}

pub fn run_benchmark_suite(
    functions: Vec<fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>>,
    names: Vec<&str>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>
) {
    let mut results: Vec<Metrics> = vec![];
    for iter_tuple in functions.iter().zip(names.iter()) {
        let (function, name) = iter_tuple;
        results.push(benchmark_function(*function, data, distance_matrix, name));
    }

}

pub fn save_solution(solution: Vec<usize>, path: &str) {
    let mut data_file = File::create(path).expect("creation failed");
    for point_id in solution {
        data_file
            .write_all(format!("{point_id}\n").as_bytes())
            .expect("write failed");
    }
}
