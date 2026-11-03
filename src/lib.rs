// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license. Enterprise integration without AGPL copyleft obligations requires a separate commercial license agreement; see COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

pub use hip_algebra::{MockQuaternionBackend, Quaternion, QuaternionBackend, DEFAULT_MODULUS};
pub use hip_runtime::{
    run_simulation, run_simulation_with_engine, BulkExecutionEngine, MockBulkExecutionEngine,
    RunReport,
};
