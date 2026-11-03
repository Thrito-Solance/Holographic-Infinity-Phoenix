// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license. Enterprise integration without AGPL copyleft obligations requires a separate commercial license agreement; see COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use hip_algebra::{NonInvertibleQuaternion, Quaternion, DEFAULT_MODULUS};

pub fn disentangle(
    basis: Quaternion,
    input: Quaternion,
) -> Result<Quaternion, NonInvertibleQuaternion> {
    Ok(basis
        .hamilton_product(input)
        .hamilton_product(basis.inverse()?))
}

pub fn isometry(left: Quaternion, right: Quaternion) -> Quaternion {
    left.modular_sum(right).scale(DEFAULT_MODULUS.div_ceil(2))
}

pub trait MeraExecutionBackend {
    fn disentangle(
        &self,
        basis: Quaternion,
        input: Quaternion,
    ) -> Result<Quaternion, NonInvertibleQuaternion>;
    fn isometry(&self, left: Quaternion, right: Quaternion) -> Quaternion;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MockMeraBackend;

impl MeraExecutionBackend for MockMeraBackend {
    fn disentangle(
        &self,
        basis: Quaternion,
        input: Quaternion,
    ) -> Result<Quaternion, NonInvertibleQuaternion> {
        disentangle(basis, input)
    }

    fn isometry(&self, left: Quaternion, right: Quaternion) -> Quaternion {
        isometry(left, right)
    }
}

#[cfg(feature = "enterprise-core")]
pub mod enterprise {
    pub use super::{MeraExecutionBackend, MockMeraBackend};
}

#[cfg(test)]
mod tests {
    use super::{disentangle, isometry, MeraExecutionBackend, MockMeraBackend};
    use hip_algebra::Quaternion;

    #[test]
    fn isometry_averages_in_modular_arithmetic() {
        let left = Quaternion::new(8, 0, 0, 0);
        let right = Quaternion::new(10, 0, 0, 0);
        assert_eq!(isometry(left, right), Quaternion::new(9, 0, 0, 0));
    }

    #[test]
    fn disentangler_rejects_noninvertible_basis() {
        let basis = Quaternion::new(1, 68_090, 0, 0);
        assert!(disentangle(basis, Quaternion::new(1, 0, 0, 0)).is_err());
    }

    #[test]
    fn mock_backend_implements_public_execution_trait() {
        let backend = MockMeraBackend;
        let value = Quaternion::new(3, 0, 0, 0);
        assert_eq!(backend.isometry(value, value), isometry(value, value));
    }
}
