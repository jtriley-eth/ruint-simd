use crate::U256x4;
use ruint::aliases::U256;
use std::simd::Simd;

impl From<[U256; 4]> for U256x4 {
    fn from(array: [U256; 4]) -> Self {
        let limbs_0 = array[0].into_limbs();
        let limbs_1 = array[1].into_limbs();
        let limbs_2 = array[2].into_limbs();
        let limbs_3 = array[3].into_limbs();
        Self {
            limbs: [
                Simd::from_array([limbs_0[0], limbs_1[0], limbs_2[0], limbs_3[0]]),
                Simd::from_array([limbs_0[1], limbs_1[1], limbs_2[1], limbs_3[1]]),
                Simd::from_array([limbs_0[2], limbs_1[2], limbs_2[2], limbs_3[2]]),
                Simd::from_array([limbs_0[3], limbs_1[3], limbs_2[3], limbs_3[3]]),
            ],
        }
    }
}
