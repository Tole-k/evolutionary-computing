mod utils;
use utils::DataPoint;

fn main() {
    let records: Vec<DataPoint> = utils::load_data("../data/TSPA.csv");
    let random_solution = utils::generate_random_solution(200);
    for value in &random_solution {
        println!("{}", value);
    }
    let total_score = utils::check_solution(random_solution, records);
    println!("Total cost {total_score:.1}");
}
