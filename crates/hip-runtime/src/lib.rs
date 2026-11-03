// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license. Enterprise integration without AGPL copyleft obligations requires a separate commercial license agreement; see COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_algebra::Quaternion;
use tokio::sync::mpsc;

pub trait BulkExecutionEngine {
    fn advance(&mut self, state: Quaternion) -> Quaternion;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MockBulkExecutionEngine;

impl BulkExecutionEngine for MockBulkExecutionEngine {
    fn advance(&mut self, state: Quaternion) -> Quaternion {
        let (a, b, c, d) = state.components();
        Quaternion::new(
            (a * 3 + 1) % hip_algebra::DEFAULT_MODULUS,
            (b + 1) % hip_algebra::DEFAULT_MODULUS,
            c,
            d,
        )
    }
}

#[cfg(feature = "enterprise-core")]
pub mod enterprise {
    pub use super::{BulkExecutionEngine, MockBulkExecutionEngine};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunReport {
    pub steps: usize,
    pub delivered: usize,
    pub dropped: usize,
    pub constraints_received: usize,
    pub constraints_dropped: usize,
    pub final_state: Quaternion,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Forward { step: usize, state: Quaternion },
    Constraint { step: usize, state: Quaternion },
}

pub async fn run_simulation(
    initial_state: Quaternion,
    steps: usize,
    drop_every: Option<usize>,
) -> Result<RunReport, tokio::task::JoinError> {
    run_simulation_with_engine(initial_state, steps, drop_every, MockBulkExecutionEngine).await
}

pub async fn run_simulation_with_engine<E: BulkExecutionEngine>(
    initial_state: Quaternion,
    steps: usize,
    drop_every: Option<usize>,
    mut engine: E,
) -> Result<RunReport, tokio::task::JoinError> {
    let (to_boundary, mut boundary_rx) = mpsc::channel::<Message>(32);
    let (to_bulk, mut bulk_rx) = mpsc::channel::<Message>(32);

    let boundary = tokio::spawn(async move {
        let mut delivered = 0;
        let mut dropped = 0;
        let mut constraints_dropped = 0;
        let mut received = 0;
        while let Some(Message::Forward { step, state }) = boundary_rx.recv().await {
            received += 1;
            if drop_every.is_some_and(|period| period > 0 && received % period != 0) {
                dropped += 1;
                continue;
            }
            delivered += 1;
            if to_bulk
                .try_send(Message::Constraint { step, state })
                .is_err()
            {
                constraints_dropped += 1;
            }
        }
        (delivered, dropped, constraints_dropped)
    });

    let mut state = initial_state;
    let mut constraints_received = 0;
    for step in 0..steps {
        if to_boundary
            .send(Message::Forward { step, state })
            .await
            .is_err()
        {
            break;
        }
        while let Ok(message) = bulk_rx.try_recv() {
            if let Message::Constraint {
                step: response_step,
                state: response,
            } = message
            {
                if response_step == step {
                    let _validated_response = response;
                    constraints_received += 1;
                }
            }
        }
        state = engine.advance(state);
    }

    drop(to_boundary);
    let (delivered, dropped, constraints_dropped) = boundary.await?;
    Ok(RunReport {
        steps,
        delivered,
        dropped,
        constraints_received,
        constraints_dropped,
        final_state: state,
    })
}

#[cfg(test)]
mod tests {
    use super::run_simulation;
    use hip_algebra::{Quaternion, DEFAULT_MODULUS};

    #[tokio::test]
    async fn applies_deterministic_ninety_percent_loss() {
        let report = match run_simulation(Quaternion::new(1, 2, 3, 4), 100, Some(10)).await {
            Ok(report) => report,
            Err(error) => panic!("actor task failed: {error}"),
        };
        assert_eq!(report.steps, 100);
        assert_eq!(report.delivered, 10);
        assert_eq!(report.dropped, 90);
        assert!(report.final_state.components().0 < DEFAULT_MODULUS);
    }

    #[tokio::test]
    async fn zero_steps_preserve_initial_state() {
        let initial = Quaternion::new(1, 2, 3, 4);
        let report = match run_simulation(initial, 0, None).await {
            Ok(report) => report,
            Err(error) => panic!("actor task failed: {error}"),
        };
        assert_eq!(report.final_state, initial);
    }
}
