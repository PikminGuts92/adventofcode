#[cfg(test)] mod data;

pub struct LanternFish {
    pub timer: u32,
}

impl LanternFish {
    pub fn new(timer: u32) -> LanternFish {
        LanternFish {
            timer,
        }
    }

    pub fn spawn_offspring(&self) -> LanternFish {
        LanternFish {
            timer: 8
        }
    }

    pub fn end_day(&mut self) {
        self.timer = match self.timer {
            0 => 6,
            _ => self.timer - 1,
        };
    }

    pub fn is_fertile(&self) -> bool {
        self.timer.eq(&0)
    }
}

pub fn simulate_population_growth(state: &[u32], mut days: u32) -> u64 {
    let mut fishes = state
        .iter()
        .map(|s| LanternFish::new(*s))
        .collect::<Vec<_>>();

    while days > 0 {
        let mut new_fishes = Vec::new();

        for fish in fishes.iter_mut() {
            // Spawn new fish if fertile
            if fish.is_fertile() {
                let offspring = fish.spawn_offspring();
                new_fishes.push(offspring);
            }

            // Update day
            fish.end_day();
        }

        // Update fishes
        if !new_fishes.is_empty() {
            fishes.append(&mut new_fishes);
        }

        days -= 1;
    }

    fishes.len() as u64
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 18, 26)]
    #[case(TEST_DATA_1, 80, 5934)]
    #[case(TEST_DATA_1, 256, 26984457539)]
    #[case(TEST_DATA_2, 80, 391888)]
    //#[case(TEST_DATA_1, true, 12)]
    //#[case(TEST_DATA_2, true, 19663)]
    pub fn simulate_population_growth_test<T: AsRef<[u32]>>(#[case] state: T, #[case] days: u32, #[case] expected: u64) {
        let result = simulate_population_growth(state.as_ref(), days);
        assert_eq!(expected, result);
    }
}