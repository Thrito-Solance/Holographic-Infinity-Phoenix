// Copyright (c) 2026 HIP contributors.
// Dual-licensed under AGPL-3.0-only or a separate commercial license. Enterprise integration without AGPL copyleft obligations requires a separate commercial license agreement; see COMMERCIAL_LICENSING.md.
#![deny(clippy::unwrap_used)]

use core::fmt;

pub const DEFAULT_MODULUS: u64 = 104_729;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonInvertibleQuaternion;

impl fmt::Display for NonInvertibleQuaternion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("quaternion has zero norm modulo the modulus")
    }
}

impl std::error::Error for NonInvertibleQuaternion {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quaternion {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl Quaternion {
    pub const fn zero() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    pub fn new(a: u64, b: u64, c: u64, d: u64) -> Self {
        Self {
            a: a % DEFAULT_MODULUS,
            b: b % DEFAULT_MODULUS,
            c: c % DEFAULT_MODULUS,
            d: d % DEFAULT_MODULUS,
        }
    }

    pub const fn components(self) -> (u64, u64, u64, u64) {
        (self.a, self.b, self.c, self.d)
    }

    pub fn modular_sum(self, other: Self) -> Self {
        Self::new(
            add_mod(self.a, other.a),
            add_mod(self.b, other.b),
            add_mod(self.c, other.c),
            add_mod(self.d, other.d),
        )
    }

    pub fn modular_difference(self, other: Self) -> Self {
        Self::new(
            sub_mod(self.a, other.a),
            sub_mod(self.b, other.b),
            sub_mod(self.c, other.c),
            sub_mod(self.d, other.d),
        )
    }

    pub fn hamilton_product(self, other: Self) -> Self {
        let aa = mul_mod(self.a, other.a);
        let bb = mul_mod(self.b, other.b);
        let cc = mul_mod(self.c, other.c);
        let dd = mul_mod(self.d, other.d);
        let ab = mul_mod(self.a, other.b);
        let ba = mul_mod(self.b, other.a);
        let cd = mul_mod(self.c, other.d);
        let dc = mul_mod(self.d, other.c);
        let ac = mul_mod(self.a, other.c);
        let bd = mul_mod(self.b, other.d);
        let ad = mul_mod(self.a, other.d);
        let bc = mul_mod(self.b, other.c);

        Self {
            a: sub_mod(sub_mod(sub_mod(aa, bb), cc), dd),
            b: add_mod(add_mod(add_mod(ab, ba), cd), sub_mod(0, dc)),
            c: add_mod(
                add_mod(sub_mod(ac, bd), mul_mod(self.c, other.a)),
                mul_mod(self.d, other.b),
            ),
            d: add_mod(
                add_mod(sub_mod(ad, bc), mul_mod(self.c, other.b)),
                mul_mod(self.d, other.a),
            ),
        }
    }

    pub fn conjugate(self) -> Self {
        Self {
            a: self.a,
            b: sub_mod(0, self.b),
            c: sub_mod(0, self.c),
            d: sub_mod(0, self.d),
        }
    }

    pub fn norm_sq(self) -> u64 {
        add_mod(
            add_mod(mul_mod(self.a, self.a), mul_mod(self.b, self.b)),
            add_mod(mul_mod(self.c, self.c), mul_mod(self.d, self.d)),
        )
    }

    pub fn inverse(self) -> Result<Self, NonInvertibleQuaternion> {
        let norm = self.norm_sq();
        if norm == 0 {
            return Err(NonInvertibleQuaternion);
        }
        let norm_inverse = modular_pow(norm, DEFAULT_MODULUS - 2, DEFAULT_MODULUS);
        Ok(self.conjugate().scale(norm_inverse))
    }

    pub fn scale(self, scalar: u64) -> Self {
        Self {
            a: mul_mod(self.a, scalar % DEFAULT_MODULUS),
            b: mul_mod(self.b, scalar % DEFAULT_MODULUS),
            c: mul_mod(self.c, scalar % DEFAULT_MODULUS),
            d: mul_mod(self.d, scalar % DEFAULT_MODULUS),
        }
    }
}

pub trait QuaternionBackend {
    fn add(&self, left: Quaternion, right: Quaternion) -> Quaternion;
    fn multiply(&self, left: Quaternion, right: Quaternion) -> Quaternion;
    fn inverse(&self, value: Quaternion) -> Result<Quaternion, NonInvertibleQuaternion>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MockQuaternionBackend;

impl QuaternionBackend for MockQuaternionBackend {
    fn add(&self, left: Quaternion, right: Quaternion) -> Quaternion {
        left.modular_sum(right)
    }

    fn multiply(&self, left: Quaternion, right: Quaternion) -> Quaternion {
        left.hamilton_product(right)
    }

    fn inverse(&self, value: Quaternion) -> Result<Quaternion, NonInvertibleQuaternion> {
        value.inverse()
    }
}

#[cfg(feature = "enterprise-core")]
pub mod enterprise {
    pub use super::{MockQuaternionBackend, QuaternionBackend};
}

fn modular_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus <= 1 {
        return 0;
    }
    base %= modulus;
    let mut result = 1 % modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = ((u128::from(result) * u128::from(base)) % u128::from(modulus)) as u64;
        }
        base = ((u128::from(base) * u128::from(base)) % u128::from(modulus)) as u64;
        exponent >>= 1;
    }
    result
}

fn add_mod(left: u64, right: u64) -> u64 {
    (left + right) % DEFAULT_MODULUS
}

fn sub_mod(left: u64, right: u64) -> u64 {
    (left + DEFAULT_MODULUS - right) % DEFAULT_MODULUS
}

fn mul_mod(left: u64, right: u64) -> u64 {
    ((u128::from(left) * u128::from(right)) % u128::from(DEFAULT_MODULUS)) as u64
}

#[cfg(test)]
mod tests {
    use super::{
        modular_pow, MockQuaternionBackend, NonInvertibleQuaternion, Quaternion, QuaternionBackend,
        DEFAULT_MODULUS,
    };

    #[test]
    fn inverse_multiplies_to_identity() {
        let value = Quaternion::new(4, 9, 2, 7);
        let inverse = match value.inverse() {
            Ok(inverse) => inverse,
            Err(error) => panic!("unexpected non-invertible test value: {error}"),
        };
        let identity = value.hamilton_product(inverse);
        assert_eq!(identity, Quaternion::new(1, 0, 0, 0));
    }

    #[test]
    fn multiplication_is_not_commutative() {
        let i = Quaternion::new(0, 1, 0, 0);
        let j = Quaternion::new(0, 0, 1, 0);
        assert_ne!(i.hamilton_product(j), j.hamilton_product(i));
    }

    #[test]
    fn rejects_zero_norm_nonzero_quaternion() {
        let zero_divisor = Quaternion::new(1, 68_090, 0, 0);
        assert_eq!(zero_divisor.norm_sq(), 0);
        assert_eq!(zero_divisor.inverse(), Err(NonInvertibleQuaternion));
    }

    #[test]
    fn rejects_zero_quaternion_inverse() {
        assert_eq!(Quaternion::zero().inverse(), Err(NonInvertibleQuaternion));
    }

    #[test]
    fn fermat_inverse_is_valid_for_default_modulus() {
        assert_eq!(
            modular_pow(2, DEFAULT_MODULUS - 2, DEFAULT_MODULUS) * 2 % DEFAULT_MODULUS,
            1
        );
    }

    #[test]
    fn mock_backend_implements_public_arithmetic_trait() {
        let backend = MockQuaternionBackend;
        let left = Quaternion::new(1, 0, 0, 0);
        let right = Quaternion::new(2, 0, 0, 0);
        assert_eq!(backend.add(left, right), Quaternion::new(3, 0, 0, 0));
        assert_eq!(backend.multiply(left, right), Quaternion::new(2, 0, 0, 0));
    }
}
