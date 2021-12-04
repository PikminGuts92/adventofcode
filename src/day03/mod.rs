#[cfg(test)] mod data;

use std::collections::HashSet;
use std::mem::size_of;

const BITCOUNT_U32: usize = size_of::<u32>() * 8;
const BIT_1: u32 = 0b1 << 31;

pub fn calculate_power_consumption(nums: &[u32], bit_count: u8) -> u32 {
    let midpoint = (nums.len() / 2) as u32;
    let mut counts = [0u32; BITCOUNT_U32];

    for n in nums {
        for (i, count) in counts.iter_mut().enumerate() {
            *count += (*n as u32 & (BIT_1 >> i)) >> (BITCOUNT_U32 - (i + 1));
        }
    }

    // Calculate gamma rate
    let mut gamma_rate = 0;
    for (i, count) in counts.iter().enumerate() {
        if count.gt(&midpoint) {
            gamma_rate |= BIT_1 >> i;
        }
    }

    // Calculate epsilon rate (inverse of gamma rate)
    let epsilon_rate = !gamma_rate & (u32::MAX >> (BITCOUNT_U32 - bit_count as usize));

    gamma_rate * epsilon_rate
}

pub fn reduce_to_rating<T: Fn(u32, u32) -> bool>(nums: &[u32], bit_count: u8, compare: T) -> u32 {
    let mut remaining = Vec::from(nums);
    let mut ones = HashSet::new();

    for b in 0..bit_count {
        let half = (remaining.len() as f32 / 2.0).ceil() as u32;
        let bit = 0b1 << (bit_count - (b + 1));

        for (i, n) in remaining.iter().enumerate() {
            if (n & bit) > 0 {
                ones.insert(i);
            }
        }

        if compare(ones.len() as u32, half) {
            for i in (0..remaining.len()).rev() {
                if !ones.contains(&i) {
                    remaining.remove(i);
                }
            }
        } else {
            for i in (0..remaining.len()).rev() {
                if ones.contains(&i) {
                    remaining.remove(i);
                }
            }
        }

        ones.drain();

        // Stop early if only one item left
        if remaining.len() == 1 {
            break
        }
    }

    *remaining.first().unwrap()
}

pub fn calculate_life_support_rating(nums: &[u32], bit_count: u8) -> u32 {
    let oxygen_rating = reduce_to_rating(nums, bit_count, |n, half| n >= half);
    let co2_rating = reduce_to_rating(nums, bit_count, |n, half| n < half);

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 5, 198)]
    #[case(TEST_DATA_2, 12, 2261546)]
    fn calculate_power_consumption_test<T: AsRef<[u32]>>(#[case] nums: T, #[case] bit_count: u8, #[case] expected: u32) {
        let result = calculate_power_consumption(nums.as_ref(), bit_count);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1, 5, 230)]
    #[case(TEST_DATA_2, 12, 6775520)]
    fn calculate_life_support_rating_test<T: AsRef<[u32]>>(#[case] nums: T, #[case] bit_count: u8, #[case] expected: u32) {
        let result = calculate_life_support_rating(nums.as_ref(), bit_count);
        assert_eq!(expected, result);
    }
}