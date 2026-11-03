// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license; see LICENSE and COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_topology::Graph;

#[test]
fn node_failure_exposes_component_change_without_claiming_homology_preservation() {
    let mut graph = Graph::new(4);
    assert!(graph.add_undirected_edge(0, 1, 1.0).is_ok());
    assert!(graph.add_undirected_edge(1, 2, 1.0).is_ok());
    assert!(graph.add_undirected_edge(2, 3, 1.0).is_ok());
    assert_eq!(graph.connected_components(), 1);

    let mut degraded = Graph::new(4);
    assert!(degraded.add_undirected_edge(0, 1, 1.0).is_ok());
    assert!(degraded.add_undirected_edge(2, 3, 1.0).is_ok());
    assert_eq!(degraded.connected_components(), 2);
}
