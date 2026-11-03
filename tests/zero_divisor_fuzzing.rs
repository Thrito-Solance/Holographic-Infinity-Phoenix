// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license; see LICENSE and COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_algebra::Quaternion;

#[test]
fn rejects_known_zero_norm_vector_and_small_edge_grid() {
    let zero_norm = Quaternion::new(1, 68_090, 0, 0);
    assert_eq!(zero_norm.norm_sq(), 0);
    assert!(zero_norm.inverse().is_err());

    for a in 0..8 {
        for b in 0..8 {
            for c in 0..8 {
                for d in 0..8 {
                    let value = Quaternion::new(a, b, c, d);
                    match value.inverse() {
                        Ok(inverse) => {
                            assert_eq!(
                                value.hamilton_product(inverse),
                                Quaternion::new(1, 0, 0, 0)
                            );
                        }
                        Err(_) => assert_eq!(value.norm_sq(), 0),
                    }
                }
            }
        }
    }
}
