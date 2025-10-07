use ndarray::Array2;

use crate::utils;
use crate::utils::DataPoint;

fn find_closest(
    point_id: usize,
    data: Vec<DataPoint>,
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
    data: Vec<DataPoint>,
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
    data: Vec<DataPoint>,
    starting_point_id: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut last_point_id = starting_point_id;
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    for _ in 1..data.len() / 2 {
        let (closest_point_id, _) =
            find_closest(last_point_id, not_visited_points.clone(), distance_matrix);
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
    data: Vec<DataPoint>,
    starting_point_id: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    tsp_path.push(starting_point_id);
    not_visited_points.remove(starting_point_id);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let (mut closest_point_id, mut closest_distance) =
            find_closest(tsp_path[0], not_visited_points.clone(), distance_matrix);
        for (i, pair) in tsp_path.clone().windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            let distance: f64;
            let point_id: usize;
            (point_id, distance) =
                find_cheapest_extension(a, b, not_visited_points.clone(), distance_matrix);
            if distance < closest_distance {
                closest_distance = distance;
                insert_spot = i + 1;
                closest_point_id = point_id;
            }
        }
        let (point_id, distance) = find_closest(
            tsp_path[tsp_path.len() - 1],
            not_visited_points.clone(),
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
    data: Vec<DataPoint>,
    starting_point_id: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    tsp_path.push(starting_point_id);
    not_visited_points.remove(starting_point_id);
    for _ in 1..data.len() / 2 {
        let mut insert_spot: usize = 0;
        let mut closest_point_id = starting_point_id;
        let mut closest_distance = f64::INFINITY;
        for (i, pair) in tsp_path.clone().windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            let distance: f64;
            let point_id: usize;
            (point_id, distance) =
                find_cheapest_extension(a, b, not_visited_points.clone(), distance_matrix);
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
            find_cheapest_extension(a, b, not_visited_points.clone(), distance_matrix);
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
    let distance_matrix = utils::calculate_distance_matrix(data.clone());
    let random_solution = utils::generate_random_solution(data.len());
    let total_score = utils::check_solution(random_solution, data.clone(), &distance_matrix);
    println!("Total cost from random solution: {total_score:.1}");

    let metric_nn_tlp =
        utils::benchmark_function(greedy_nn_to_last_point, data.clone(), &distance_matrix);
    println!(
        "NN to last point (min: {}, avg: {}, max: {})",
        metric_nn_tlp.min, metric_nn_tlp.avg, metric_nn_tlp.max,
    );
    utils::save_solution(
        metric_nn_tlp.best_solution,
        "../reports/report1/greedy_nn_to_last_point.csv",
    );

    let metric_nn_tc =
        utils::benchmark_function(greedy_nn_to_cycle, data.clone(), &distance_matrix);
    println!(
        "NN to cycle (min: {}, avg: {}, max: {})",
        metric_nn_tc.min, metric_nn_tc.avg, metric_nn_tc.max,
    );
    utils::save_solution(
        metric_nn_tc.best_solution,
        "../reports/report1/greedy_nn_to_cycle.csv",
    );
        let metric_greedy_cycle =
        utils::benchmark_function(greedy_cycle, data.clone(), &distance_matrix);
    println!(
        "Greedy cycle (min: {}, avg: {}, max: {})",
        metric_greedy_cycle.min, metric_greedy_cycle.avg, metric_greedy_cycle.max,
    );
    utils::save_solution(
        metric_greedy_cycle.best_solution,
        "../reports/report1/greedy_cycle.csv",
    );
}
