use ndarray::Array2;
use clap::Parser;

use std::time::Instant;

use crate::utils;
use crate::utils::DataPoint;

fn find_closest(
    point_id: usize,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
) -> (usize, f64) {
    let mut closest_distance: f64 = f64::INFINITY;
    let mut closest_point_id: usize = point_id;
    for candidate_point in data {
        let current_distance = distance_matrix[[point_id, candidate_point.id]];
        if current_distance < closest_distance {
            closest_point_id = candidate_point.id;
            closest_distance = current_distance;
        }
    }
    (closest_point_id, closest_distance)
}

fn find_cheapest_extension(
    point_a_id: usize,
    point_b_id: usize,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
) -> (usize, f64) {
    let mut closest_distance: f64 = f64::INFINITY;
    let mut closest_point: usize = point_a_id;
    for candidate_point in data {
        let current_distance = distance_matrix[[point_a_id, candidate_point.id]]
            + distance_matrix[[candidate_point.id, point_b_id]]
            - distance_matrix[[point_a_id, point_b_id]];
        if current_distance < closest_distance {
            closest_point = candidate_point.id;
            closest_distance = current_distance;
        }
    }
    (closest_point, closest_distance)
}

fn greedy_nn_to_last_point(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut last_point_id = data[starting_point_index].id;
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    for _ in 1..data.len() / 2 {
        let (closest_point_id, _) =
            find_closest(last_point_id, &not_visited_points, distance_matrix);
        tsp_path.push(closest_point_id);
        let index = not_visited_points
            .iter()
            .position(|n| n.id == closest_point_id)
            .unwrap();
        not_visited_points.remove(index);
        last_point_id = closest_point_id;
    }
    tsp_path
}

fn greedy_nn_to_cycle(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let starting_point_id = data[starting_point_index].id;
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    tsp_path.push(starting_point_id);
    not_visited_points.remove(starting_point_index);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let (mut closest_point_id, mut closest_distance) =
            find_closest(tsp_path[0], &not_visited_points, distance_matrix);
        for (i, pair) in tsp_path.windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            let distance: f64;
            let point_id: usize;
            (point_id, distance) =
                find_cheapest_extension(a, b, &not_visited_points, distance_matrix);
            if distance < closest_distance {
                closest_distance = distance;
                insert_spot = i + 1;
                closest_point_id = point_id;
            }
        }
        let (point_id, distance) = find_closest(
            tsp_path[tsp_path.len() - 1],
            &not_visited_points,
            distance_matrix,
        );
        if distance < closest_distance {
            closest_point_id = point_id;
            tsp_path.push(closest_point_id);
        } else {
            tsp_path.insert(insert_spot, closest_point_id);
        }
        let index = not_visited_points
            .iter()
            .position(|n| n.id == closest_point_id)
            .unwrap();
        not_visited_points.remove(index);
    }
    tsp_path
}

fn greedy_cycle(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let starting_point_id = data[starting_point_index].id;
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    tsp_path.push(starting_point_id);
    not_visited_points.remove(starting_point_index);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let mut closest_point_id = starting_point_id;
        let mut closest_distance = f64::INFINITY;
        for (i, pair) in tsp_path.windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            let distance: f64;
            let point_id: usize;
            (point_id, distance) =
                find_cheapest_extension(a, b, &not_visited_points, distance_matrix);
            if distance < closest_distance {
                closest_distance = distance;
                insert_spot = i + 1;
                closest_point_id = point_id;
            }
        }
        let a = tsp_path[tsp_path.len() - 1];
        let b = tsp_path[0];
        let distance: f64;
        let point_id: usize;
        (point_id, distance) =
            find_cheapest_extension(a, b, &not_visited_points, distance_matrix);
        if distance < closest_distance {
            closest_point_id = point_id;
            tsp_path.push(closest_point_id);
        } else {
            tsp_path.insert(insert_spot, closest_point_id);
        }
        let index = not_visited_points
            .iter()
            .position(|n| n.id == closest_point_id)
            .unwrap();
        not_visited_points.remove(index);
    }
    tsp_path
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    mode: String,

    /// Number of times to greet
    #[arg(short, long)]
    algorithm: String,

    /// Index of staring point
    #[arg(short, long, default_value_t=1)]
    starting_point: usize,
}

pub fn main() {
    let data: Vec<DataPoint> = utils::load_data("../data/TSPB.csv");
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let args = Args::parse();
    let mode = args.mode;
    let algorithm = args.algorithm.as_str();
    let benchmark_suite = |name:&str,f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>|
    {
        let now = Instant::now();
        let metric =
        utils::benchmark_function(f, &data, &distance_matrix);
        println!(
        "{name} (min: {}, avg: {}, max: {})",
        metric.min, metric.avg, metric.max,
        );
        utils::save_solution(
            metric.best_solution,
        format!("../reports/report1/{name}.csv").as_str(),
        );
        let milisecs = now.elapsed().as_secs_f64()*1000f64;
        println!("Elapsed: {:.6?}", milisecs);
    };

    let single_run = |name:&str,starting_point: usize,f:fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>|
    {
        let solution = f(&data, starting_point, &distance_matrix);
        let solution_score = utils::check_solution(&solution, &data, &distance_matrix);
        println!("{name} solution score: {solution_score}")
    };
    if mode == "benchmark"
    {
        match algorithm {
            "random" => benchmark_suite("random",utils::generate_random_solution),
            "nn-to-last-point" => benchmark_suite("nn_to_last_point",greedy_nn_to_last_point),
            "nn-to-cycle" => benchmark_suite("nn_to_cycle", greedy_nn_to_cycle),
            "greedy-cycle" => benchmark_suite("greedy_cycle", greedy_cycle),
            "all" => {
                benchmark_suite("random",utils::generate_random_solution);
                benchmark_suite("nn_to_last_point",greedy_nn_to_last_point);
                benchmark_suite("nn_to_cycle",greedy_nn_to_cycle);
                benchmark_suite("greedy_cycle",greedy_cycle);
            }
            _ => println!("Invalid algorithm")
        }
    }
    else if mode == "single_run" {
        let starting_point = args.starting_point;
        match algorithm {
            "random" => single_run("random",starting_point,utils::generate_random_solution),
            "nn-to-last-point" => single_run("nn_to_last_point",starting_point,greedy_nn_to_last_point),
            "nn-to-cycle" => single_run("nn_to_cycle",starting_point, greedy_nn_to_cycle),
            "greedy-cycle" => single_run("greedy_cycle",starting_point, greedy_cycle),
            "all" => {
                single_run("random",starting_point,utils::generate_random_solution);
                single_run("nn_to_last_point",starting_point,greedy_nn_to_last_point);
                single_run("nn_to_cycle",starting_point,greedy_nn_to_cycle);
                single_run("greedy_cycle",starting_point,greedy_cycle);
            }
            _ => println!("Invalid algorithm")
        }
    }
    else {
        println!("Invalid mode")
    }

}
