mod greedy_algorithms;
mod utils;

fn main() {
    let data: Vec<utils::DataPoint> = utils::load_data("../data/TSPB.csv");
    let distance_matrix = utils::calculate_distance_matrix(&data);
    utils::run_benchmark_suite(vec![greedy_algorithms::greedy_cycle, greedy_algorithms::greedy_nn_to_cycle, greedy_algorithms::greedy_nn_to_last_point ],
        vec!["cycle", "nn_cycle", "nn_to_last_point"], &data, &distance_matrix);
    greedy_algorithms::main();
}
