use crate::utils::{DataPoint, calculate_distance};

fn find_closest(point: DataPoint, data: Vec<DataPoint>) -> (DataPoint, f64) {
    let mut closest_distance: f64 = f64::INFINITY;
    let mut closest_point: DataPoint = point;
    for candidate_point in data {
        let current_distance = calculate_distance(point, candidate_point);
        if current_distance < closest_distance {
            closest_point = candidate_point;
            closest_distance = current_distance;
        }
    }
    (closest_point, closest_distance)
}

pub fn greedy_nn_to_last_point(data: Vec<DataPoint>, starting_point_id: usize) -> Vec<usize> {
    let starting_point = data[starting_point_id];
    let mut last_point = starting_point;
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    for _ in 1..data.len() / 2 {
        let (closest_point, _) = find_closest(last_point, not_visited_points.clone());
        tsp_path.push(closest_point.id);
        let index = not_visited_points
            .iter()
            .position(|n| n.id == closest_point.id)
            .unwrap();
        not_visited_points.remove(index);
        last_point = closest_point;
    }
    tsp_path
}

pub fn greedy_nn_to_cycle(data: Vec<DataPoint>, starting_point_id: usize) -> Vec<usize> {
    let starting_point = data[starting_point_id];
    let mut tsp_path: Vec<usize> = vec![];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    tsp_path.push(starting_point_id);
    not_visited_points.remove(starting_point_id);
    for _ in 1..data.len() / 2 {
        let mut closest_distance: f64 = f64::INFINITY;
        let mut closest_point: DataPoint = starting_point;
        for i in tsp_path.clone() {
            let distance: f64;
            let point: DataPoint;
            (point, distance) = find_closest(data[i], not_visited_points.clone());
            if distance < closest_distance {
                closest_distance = distance;
                closest_point = point;
            }
        }
        // TODO: Currently algorithm adds to the end the point that is the closest to any of points in cycle.
        // This is highly sub-optimal and I'm not sure if it's how this algorithm is meant to be.
        // We could find the closest point in the graph to it, and put it this way
        // but then what is the difference between this and greedy cycle algorithm?

        tsp_path.push(closest_point.id);

        let index = not_visited_points
            .iter()
            .position(|n| n.id == closest_point.id)
            .unwrap();
        not_visited_points.remove(index);
    }
    tsp_path
}

pub fn greedy_cycle(data: Vec<DataPoint>, starting_point: usize) -> Vec<usize> {
    // TODO: Implement
    (_, _) = (data, starting_point);
    vec![1, 2]
}
