use std::collections::HashMap;
use std::io::Write;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::python3api::Python;
use crate::Error;

const SCRIPT: &str = include_str!("../../algo/max_flow_min_cost/negative_cycle_removal.py");
const FILENAME: &str = "flow.txt";

#[derive(Debug, Deserialize, Serialize)]
pub struct Edge {
    to: usize,
    capacity: u64,
    #[serde(default)]
    used_capacity: u64,
    cost: u64,
}

type RawOutput = (HashMap<String, HashMap<String, [u64; 3]>>, u64, u64);

#[tauri::command]
pub async fn run_max_flow_min_cost(
    handle: AppHandle,
    edges: Vec<Vec<Edge>>,
) -> Result<(Vec<Vec<Edge>>, u64, u64), Error> {
    let path = handle
        .path_resolver()
        .app_local_data_dir()
        .and_then(|mut path| {
            std::fs::create_dir_all(&path).ok().map(|_| {
                path.push(FILENAME);
                path
            })
        })
        .ok_or(Error::NoPath)?;

    let path_str = path.to_str().ok_or(Error::NoPath)?;

    let mut file = std::fs::File::create(&path)?;

    for (index, edges) in edges.into_iter().enumerate() {
        file.write_all(format!("{}:", index + 1).as_bytes())?;
        for edge in edges {
            file.write_all(format!(" {}:{},{}", edge.to, edge.capacity, edge.cost).as_bytes())?;
        }
        file.write_all(b"\n")?;
    }

    let (graph, flow, cost): RawOutput = Python::eval_with_gil(SCRIPT, "start", path_str)?;

    let mut result = Vec::with_capacity(graph.len());

    for index in 1..=graph.len() {
        let mut result_edges = Vec::new();
        for (to, edge) in &graph[&index.to_string()] {
            if edge[0] != 0 {
                result_edges.push(Edge {
                    to: to.parse()?,
                    capacity: edge[0],
                    used_capacity: edge[1],
                    cost: edge[2],
                });
            }
        }
        result.push(result_edges);
    }

    Ok((result, flow, cost))
}
