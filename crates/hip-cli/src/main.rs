// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license; see LICENSE and COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_algebra::Quaternion;
use hip_runtime::run_simulation;

#[tokio::main]
async fn main() {
    let steps = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(100);
    let report = match run_simulation(Quaternion::new(1, 1, 1, 1), steps, None).await {
        Ok(report) => report,
        Err(error) => {
            eprintln!("simulation actor failed: {error}");
            std::process::exit(1);
        }
    };
    println!(
        "steps={} delivered={} dropped={} final={:?}",
        report.steps, report.delivered, report.dropped, report.final_state
    );
}
