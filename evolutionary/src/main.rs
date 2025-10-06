mod utils;
use utils::DataPoint;
mod greedy_algorithms;

fn main() {
    let records: Vec<DataPoint> = utils::load_data("../data/TSPA.csv");
    let random_solution = utils::generate_random_solution(10);
    let total_score = utils::check_solution(random_solution, records.clone());
    println!("Total cost from random solution: {total_score:.1}");

    let greedy_solution = greedy_algorithms::greedy_nn_to_last_point(records.clone(), 0);
    let total_score = utils::check_solution(greedy_solution.clone(), records.clone());
    println!("Total cost from greedy solution: {total_score:.1}");
    for point in greedy_solution {
        print!("{point} ")
    }
    println!();

    let greedy_solution = greedy_algorithms::greedy_nn_to_cycle(records.clone(), 0);
    let total_score = utils::check_solution(greedy_solution, records.clone());
    println!("Total cost from greedy solution: {total_score:.1}");
}
