pub fn calculate_power_consumption(nums: &[u8]) -> u32 {
    let midpoint = (nums.len() / 2) as u32;
    let mut counts = [0u32; 5];

    for n in nums {
        for (i, count) in counts.iter_mut().enumerate() {
            *count += (*n as u32 & (0b10000 >> i)) >> (4 - i);
        }
    }

    let mut gamma_rate = 0;
    let epsilon_rate;

    for (i, count) in counts.iter().enumerate() {
        if count.gt(&midpoint) {
            gamma_rate |= 0b10000 >> i;
        }
    }

    epsilon_rate = !gamma_rate & 0b11111;

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case([0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010], 198)]
    fn calculate_power_consumption_test<T: AsRef<[u8]>>(#[case] nums: T, #[case] expected: u32) {
        let result = calculate_power_consumption(nums.as_ref());
        assert_eq!(expected, result);
    }
}