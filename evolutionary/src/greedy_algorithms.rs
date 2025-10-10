use ndarray::Array2;

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



pub fn main() {
    let data: Vec<DataPoint> = utils::load_data("../data/TSPB.csv");
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let algorithm = std::env::args().nth(1).expect("please specify algorithm");
    let algorithm = algorithm.as_str();
    let full_suite = |name:&str,f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>|
    {
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
    };
    match algorithm {
        "random" => full_suite("random",utils::generate_random_solution),
        "nn-to-last-point" => full_suite("nn_to_last_point",greedy_nn_to_last_point),
        "nn-to-cycle" => full_suite("nn_to_cycle", greedy_nn_to_cycle),
        "greedy-cycle" => full_suite("greedy_cycle", greedy_cycle),
        "all" => {
            full_suite("random",utils::generate_random_solution);
            full_suite("nn_to_last_point",greedy_nn_to_last_point);
            full_suite("nn_to_cycle",greedy_nn_to_cycle);
            full_suite("greedy_cycle",greedy_cycle);
        }
        _ => println!("Invalid algorithm")
    }
}
