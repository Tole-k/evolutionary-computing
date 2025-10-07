use csv::ReaderBuilder;
use rand::prelude::*;
use std::cmp::Ordering;
use std::f64;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct DataPoint {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub cost: i32,
}

pub struct Metrics {
    pub _scores: Vec<f64>,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
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

pub fn benchmark_function(
    f: fn(Vec<DataPoint>, usize) -> Vec<usize>,
    data: Vec<DataPoint>,
) -> Metrics {
    let mut scores: Vec<f64> = vec![];
    for i in 0..200 {
        let solution = f(data.clone(), i);
        scores.push(check_solution(solution, data.clone()));
    }

    let min = scores.iter().fold(f64::INFINITY, |a, &b| {
        match PartialOrd::partial_cmp(&a, &b) {
            None => f64::NAN,
            Some(Ordering::Less) => a,
            Some(_) => b,
        }
    });
    let max = scores.iter().fold(-f64::INFINITY, |a, &b| {
        match PartialOrd::partial_cmp(&a, &b) {
            None => f64::NAN,
            Some(Ordering::Greater) => a,
            Some(_) => b,
        }
    });
    let sum: f64 = scores.iter().sum();
    let avg: f64 = sum / scores.len() as f64;

    // let minValue = *scores.iter().min().unwrap();
    Metrics {
        _scores: scores,
        min,
        max,
        avg,
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
