use scheduling_conflicts::{run_experiment_from_stdin, tresoldi};

fn main() {
    run_experiment_from_stdin(tresoldi).expect("Failed to run list algorithm");
}
