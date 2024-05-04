use crate::U256x4;
use std::ops::Add;
use std::simd::prelude::SimdPartialOrd;
use std::simd::Simd;

impl U256x4 {
    pub fn wrapping_add(self, rhs: Self) -> Self {
        let sum_limb0 = self.limbs[0] + rhs.limbs[0];
        let sum_limb1 = self.limbs[1] + rhs.limbs[1] + Self::carry(self.limbs[0], rhs.limbs[0]);
        let sum_limb2 = self.limbs[2] + rhs.limbs[2] + Self::carry(self.limbs[1], rhs.limbs[1]);
        let sum_limb3 = self.limbs[3] + rhs.limbs[3] + Self::carry(self.limbs[2], rhs.limbs[2]);

        Self {
            limbs: [sum_limb0, sum_limb1, sum_limb2, sum_limb3],
        }
    }

    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let sum_limb0 = self.limbs[0] + rhs.limbs[0];
        let sum_limb1 = self.limbs[1] + rhs.limbs[1] + Self::carry(self.limbs[0], rhs.limbs[0]);
        let sum_limb2 = self.limbs[2] + rhs.limbs[2] + Self::carry(self.limbs[1], rhs.limbs[1]);
        let sum_limb3 = self.limbs[3] + rhs.limbs[3] + Self::carry(self.limbs[2], rhs.limbs[2]);

        let overflow = Simd::<u64, 4>::from_array([u64::MAX; 4]) - self.limbs[3] < sum_limb3;

        (
            Self {
                limbs: [sum_limb0, sum_limb1, sum_limb2, sum_limb3],
            },
            overflow,
        )
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.overflowing_add(rhs) {
            (result, false) => Some(result),
            _ => None,
        }
    }

    pub fn saturating_add(self, rhs: Self) -> Self {
        match self.overflowing_add(rhs) {
            (result, false) => result,
            _ => Self::MAX,
        }
    }

    #[inline(always)]
    fn carry(lhs: Simd<u64, 4>, rhs: Simd<u64, 4>) -> Simd<u64, 4> {
        (Simd::<u64, 4>::from_array([u64::MAX; 4]) - lhs)
            .simd_lt(rhs)
            .select(
                Simd::<u64, 4>::from_array([1u64; 4]),
                Simd::<u64, 4>::from_array([0u64; 4]),
            )
    }
}

impl Add for U256x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}
