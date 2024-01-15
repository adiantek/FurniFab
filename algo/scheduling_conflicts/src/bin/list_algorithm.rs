use scheduling_conflicts::{list_algorithm, run_experiment_from_stdin};

fn main() {
    run_experiment_from_stdin(list_algorithm).expect("Failed to run list algorithm");
}
