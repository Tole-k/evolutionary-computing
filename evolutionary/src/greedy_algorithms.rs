use clap::Parser;
use ndarray::Array2;

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
    let mut tsp_path: Vec<usize> = vec![last_point_id];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    not_visited_points.remove(starting_point_index);
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

fn greedy_nn_to_any_point(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let starting_point_id = data[starting_point_index].id;
    let mut tsp_path: Vec<usize> = vec![starting_point_id];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    not_visited_points.remove(starting_point_index);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let mut closest_point_id = 0;
        let mut closest_distance = f64::INFINITY;
        for pos in 0..tsp_path.len() + 1 {
            let (point_id, distance);
            if pos == 0 {
                (point_id, distance) =
                    find_closest(tsp_path[0], &not_visited_points, distance_matrix);
            } else if pos == tsp_path.len() {
                (point_id, distance) = find_closest(
                    tsp_path[tsp_path.len() - 1],
                    &not_visited_points,
                    distance_matrix,
                );
            } else {
                let (a, b) = (tsp_path[pos - 1], tsp_path[pos]);
                (point_id, distance) =
                    find_cheapest_extension(a, b, &not_visited_points, distance_matrix);
            }
            if distance < closest_distance {
                closest_distance = distance;
                insert_spot = pos;
                closest_point_id = point_id;
            }
        }
        tsp_path.insert(insert_spot, closest_point_id);
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
    let mut tsp_path: Vec<usize> = vec![starting_point_id];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    not_visited_points.remove(starting_point_index);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let mut closest_point_id = starting_point_id;
        let mut closest_distance = f64::INFINITY;
        for pos in 0..tsp_path.len() + 1 {
            let (a, b);
            if pos == 0 || pos == tsp_path.len() {
                (a, b) = (tsp_path[tsp_path.len() - 1], tsp_path[0])
            } else {
                (a, b) = (tsp_path[pos - 1], tsp_path[pos]);
            }
            let (point_id, distance) =
                find_cheapest_extension(a, b, &not_visited_points, distance_matrix);
            if distance < closest_distance {
                closest_distance = distance;
                insert_spot = pos;
                closest_point_id = point_id;
            }
        }
        tsp_path.insert(insert_spot, closest_point_id);
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
    #[arg(short, long, default_value_t = 1)]
    starting_point: usize,

    ///number of nodes, -1 for all nodes from file
    #[arg(short, long, default_value_t=-1)]
    num_nodes: i32,
}

pub fn main() {
    let args = Args::parse();
    let mode = args.mode;
    let algorithm = args.algorithm.as_str();
    let nodes_subset = args.num_nodes;
    let mut data: Vec<DataPoint> = utils::load_data("data/TSPB.csv");
    if nodes_subset != -1 {
        let nodes_subset = nodes_subset as usize;
        data = data[..nodes_subset].to_vec();
    }
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let benchmark_suite =
        |name: &str, f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>| {
            let now = Instant::now();
            let metric = utils::benchmark_function(f, &data, &distance_matrix);
            println!(
                "{name} (min: {}, avg: {}, max: {})",
                metric.min, metric.avg, metric.max,
            );
            utils::save_solution(
                metric.best_solution,
                format!("reports/report1/{name}.csv").as_str(),
            );
            let milisecs = now.elapsed().as_secs_f64() * 1000f64;
            println!("Elapsed: {:.6?}", milisecs);
        };

    let single_run =
        |name: &str,
         starting_point: usize,
         f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>| {
            let solution = f(&data, starting_point, &distance_matrix);
            let solution_score = utils::check_solution(&solution, &data, &distance_matrix);
            println!("{name} solution score: {solution_score}")
        };
    if mode == "benchmark" {
        match algorithm {
            "random" => benchmark_suite("random", utils::generate_random_solution),
            "nn-to-last" => benchmark_suite("nn_to_last", greedy_nn_to_last_point),
            "nn-to-any" => benchmark_suite("nn_to_any", greedy_nn_to_any_point),
            "greedy-cycle" => benchmark_suite("greedy_cycle", greedy_cycle),
            "all" => {
                benchmark_suite("random", utils::generate_random_solution);
                benchmark_suite("nn_to_last", greedy_nn_to_last_point);
                benchmark_suite("nn_to_any", greedy_nn_to_any_point);
                benchmark_suite("greedy_cycle", greedy_cycle);
            }
            _ => println!("Invalid algorithm"),
        }
    } else if mode == "single_run" {
        let starting_point = args.starting_point;
        match algorithm {
            "random" => single_run("random", starting_point, utils::generate_random_solution),
            "nn-to-last" => {
                single_run("nn_to_last", starting_point, greedy_nn_to_last_point)
            }
            "nn-to-any" => single_run("nn_to_any", starting_point, greedy_nn_to_any_point),
            "greedy-cycle" => single_run("greedy_cycle", starting_point, greedy_cycle),
            "all" => {
                single_run("random", starting_point, utils::generate_random_solution);
                single_run("nn_to_last", starting_point, greedy_nn_to_last_point);
                single_run("nn_to_any", starting_point, greedy_nn_to_any_point);
                single_run("greedy_cycle", starting_point, greedy_cycle);
            }
            _ => println!("Invalid algorithm"),
        }
    } else {
        println!("Invalid mode")
    }
}
