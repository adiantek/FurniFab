use scheduling_conflicts::{run_experiment_from_stdin, vns};

fn main() {
    run_experiment_from_stdin(vns).expect("Failed to run VNS algorithm");
}
