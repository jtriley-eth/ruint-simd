#![feature(portable_simd)]
// #![feature(proc_macro_c_str_literals)]

pub mod add;
pub mod convert;

use ruint::aliases::U256;
use std::simd::Simd;

#[derive(Clone, Copy)]
pub struct U256x4 {
    limbs: [Simd<u64, 4>; 4],
}

impl U256x4 {
    pub const MAX: Self = Self {
        limbs: [
            Simd::from_array([u64::MAX; 4]),
            Simd::from_array([u64::MAX; 4]),
            Simd::from_array([u64::MAX; 4]),
            Simd::from_array([u64::MAX; 4]),
        ],
    };

    pub fn new(a: U256, b: U256, c: U256, d: U256) -> Self {
        let limbs_a = a.into_limbs();
        let limbs_b = b.into_limbs();
        let limbs_c = c.into_limbs();
        let limbs_d = d.into_limbs();

        Self {
            limbs: [
                Simd::from_array([limbs_a[0], limbs_b[0], limbs_c[0], limbs_d[0]]),
                Simd::from_array([limbs_a[1], limbs_b[1], limbs_c[1], limbs_d[1]]),
                Simd::from_array([limbs_a[2], limbs_b[2], limbs_c[2], limbs_d[2]]),
                Simd::from_array([limbs_a[3], limbs_b[3], limbs_c[3], limbs_d[3]]),
            ],
        }
    }

    pub fn limbs(&self) -> [Simd<u64, 4>; 4] {
        self.limbs
    }

    pub fn to_array(self) -> [U256; 4] {
        let limbs_0 = self.limbs[0].to_array();
        let limbs_1 = self.limbs[1].to_array();
        let limbs_2 = self.limbs[2].to_array();
        let limbs_3 = self.limbs[3].to_array();
        [
            U256::from_limbs([limbs_0[0], limbs_1[0], limbs_2[0], limbs_3[0]]),
            U256::from_limbs([limbs_0[1], limbs_1[1], limbs_2[1], limbs_3[1]]),
            U256::from_limbs([limbs_0[2], limbs_1[2], limbs_2[2], limbs_3[2]]),
            U256::from_limbs([limbs_0[3], limbs_1[3], limbs_2[3], limbs_3[3]]),
        ]
    }
}

impl Default for U256x4 {
    fn default() -> Self {
        Self {
            limbs: [
                Simd::from_array([0u64; 4]),
                Simd::from_array([0u64; 4]),
                Simd::from_array([0u64; 4]),
                Simd::from_array([0u64; 4]),
            ],
        }
    }
}
