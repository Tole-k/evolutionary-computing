mod utils;
use utils::DataPoint;
mod greedy_algorithms;

fn main() {
    let records: Vec<DataPoint> = utils::load_data("../data/TSPB.csv");
    let random_solution = utils::generate_random_solution(10);
    let total_score = utils::check_solution(random_solution, records.clone());
    println!("Total cost from random solution: {total_score:.1}");

    let metric_nn_tlp =
        utils::benchmark_function(greedy_algorithms::greedy_nn_to_last_point, records.clone());
    println!(
        "NN to last point (min: {}, avg: {}, max: {})",
        metric_nn_tlp.min, metric_nn_tlp.avg, metric_nn_tlp.max,
    );

    let metric_nn_tc =
        utils::benchmark_function(greedy_algorithms::greedy_nn_to_cycle, records.clone());
    println!(
        "NN to cycle (min: {}, avg: {}, max: {})",
        metric_nn_tc.min, metric_nn_tc.avg, metric_nn_tc.max,
    );
}
