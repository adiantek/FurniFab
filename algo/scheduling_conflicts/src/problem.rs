use ahash::{HashSet, HashSetExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// A task. Contains the processing time and weight of the task.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq)]
pub struct Task {
    pub processing_time: u64,
    pub weight: u64,
}

/// A conflict between two tasks described by their indices.
#[derive(Clone, Copy, Debug, Deserialize)]
struct Conflict(usize, usize);

/// A conflict graph. Contains an edge for every pair of tasks that conflict.
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "Vec<Conflict>")]
pub struct ConflictGraph {
    edges: Vec<HashSet<usize>>,
}

impl ConflictGraph {
    /// Returns whether the given tasks conflict.
    pub fn are_conflicted(&self, first: usize, second: usize) -> bool {
        self.edges
            .get(first)
            .map(|conflicts| conflicts.contains(&second))
            .unwrap_or(false)
    }

    /// Returns the conflicts of the given task.
    pub fn conflicts(&self, task: usize) -> &HashSet<usize> {
        static EMPTY: Lazy<HashSet<usize>> = Lazy::new(HashSet::new);

        self.edges.get(task).unwrap_or(&EMPTY)
    }
}

impl From<Vec<Conflict>> for ConflictGraph {
    fn from(conflicts: Vec<Conflict>) -> Self {
        let mut edges = Vec::new();

        for conflict in conflicts {
            while edges.len() <= conflict.0.max(conflict.1) {
                edges.push(HashSet::new());
            }

            edges[conflict.0].insert(conflict.1);
            edges[conflict.1].insert(conflict.0);
        }

        ConflictGraph { edges }
    }
}

/// An instance of the scheduling problem.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Instance {
    pub processors: usize,
    pub deadline: u64,
    #[serde(skip_serializing)]
    pub tasks: Vec<Task>,
    #[serde(skip_serializing)]
    pub graph: ConflictGraph,
}
