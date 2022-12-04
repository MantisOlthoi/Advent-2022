pub mod workload;

fn main() {
    let input = workload::get_input();
    let result = workload::calculate(input);

    println!("{}", result);
}
