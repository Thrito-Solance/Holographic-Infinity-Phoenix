// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license; see LICENSE and COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use core::fmt;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphError {
    InvalidNode(usize),
    InvalidWeight,
}

impl fmt::Display for GraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNode(node) => write!(formatter, "node index {node} is out of range"),
            Self::InvalidWeight => {
                formatter.write_str("edge weight must be finite and non-negative")
            }
        }
    }
}

impl std::error::Error for GraphError {}

#[derive(Debug, Clone)]
pub struct Graph {
    adjacency: Vec<Vec<f64>>,
}

impl Graph {
    pub fn new(node_count: usize) -> Self {
        Self {
            adjacency: vec![vec![0.0; node_count]; node_count],
        }
    }

    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn add_undirected_edge(
        &mut self,
        left: usize,
        right: usize,
        weight: f64,
    ) -> Result<(), GraphError> {
        if left >= self.node_count() {
            return Err(GraphError::InvalidNode(left));
        }
        if right >= self.node_count() {
            return Err(GraphError::InvalidNode(right));
        }
        if !weight.is_finite() || weight < 0.0 {
            return Err(GraphError::InvalidWeight);
        }
        self.adjacency[left][right] = weight;
        self.adjacency[right][left] = weight;
        Ok(())
    }

    pub fn normalized_laplacian(&self) -> Vec<Vec<f64>> {
        let degrees: Vec<f64> = self.adjacency.iter().map(|row| row.iter().sum()).collect();
        let count = self.node_count();
        let mut laplacian = vec![vec![0.0; count]; count];
        for row in 0..count {
            if degrees[row] == 0.0 {
                continue;
            }
            laplacian[row][row] = 1.0;
            for column in 0..count {
                if degrees[column] > 0.0 {
                    laplacian[row][column] -= self.adjacency[row][column]
                        / (degrees[row].sqrt() * degrees[column].sqrt());
                }
            }
        }
        laplacian
    }

    pub fn connected_components(&self) -> usize {
        let mut visited = vec![false; self.node_count()];
        let mut components = 0;
        for start in 0..self.node_count() {
            if visited[start] {
                continue;
            }
            components += 1;
            visited[start] = true;
            let mut queue = VecDeque::from([start]);
            while let Some(node) = queue.pop_front() {
                for (neighbor, weight) in self.adjacency[node].iter().enumerate() {
                    if *weight > 0.0 && !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        components
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn normalized_laplacian_is_symmetric() {
        let mut graph = Graph::new(3);
        assert!(graph.add_undirected_edge(0, 1, 1.0).is_ok());
        assert!(graph.add_undirected_edge(1, 2, 2.0).is_ok());
        let matrix = graph.normalized_laplacian();
        for (row_index, row) in matrix.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                assert!((*value - matrix[column_index][row_index]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn isolated_nodes_are_counted_as_components() {
        assert_eq!(Graph::new(3).connected_components(), 3);
    }
}
