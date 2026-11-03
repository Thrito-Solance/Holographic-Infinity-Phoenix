// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license; see LICENSE and COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_algebra::Quaternion;
use hip_runtime::run_simulation;

#[tokio::test]
async fn bounded_progress_under_ninety_percent_packet_loss() {
    let report = match run_simulation(Quaternion::new(5, 5, 5, 5), 50, Some(10)).await {
        Ok(report) => report,
        Err(error) => panic!("actor task failed: {error}"),
    };
    assert_eq!(report.steps, 50);
    assert_eq!(report.delivered, 5);
    assert_eq!(report.dropped, 45);
    assert!(report.final_state.components().0 < hip_algebra::DEFAULT_MODULUS);
}
