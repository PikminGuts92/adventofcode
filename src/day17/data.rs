use std::ops::RangeInclusive;

/*pub const TEST_DATA_1: [[(i64, i64); 2]; 2] = [
    [(20, -5), (30, -5)],
    [(20, -10), (30, -10)]
];*/

pub const TEST_DATA_1: [RangeInclusive<i64>; 2] = [
    20..=30, // x
    -10..=-5 // y
];

pub const TEST_DATA_2: [RangeInclusive<i64>; 2] = [
    25..=67, // x
    -260..=-200 // y
];