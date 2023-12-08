use nom::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_while, take_while1, take_until};
use nom::character::{is_alphanumeric, is_digit, is_hex_digit};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, newline, one_of, space0, space1};
use nom::combinator::{all_consuming, map, map_opt, map_parser, map_res, not, opt, peek, recognize, value};
use nom::error::{context, Error};
use nom::multi::{fold_many0, many0, separated_list0, separated_list1};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

#[derive(Debug)]
pub struct SeedMapData {
    pub seed_ids: Vec<i64>,
    pub seed_to_soil_map: Vec<[i64; 3]>,
    pub soil_to_fertilizer_map: Vec<[i64; 3]>,
    pub fertilizer_to_water_map: Vec<[i64; 3]>,
    pub water_to_light_map: Vec<[i64; 3]>,
    pub light_to_temperature_map: Vec<[i64; 3]>,
    pub temperature_to_humidity_map: Vec<[i64; 3]>,
    pub humidity_to_location_map: Vec<[i64; 3]>,
}

impl SeedMapData {
    pub fn map_seed_id_to_location(&self, seed_id: i64) -> i64 {
        let soil_id = Self::map_id(seed_id, &self.seed_to_soil_map);
        let fertilizer_id = Self::map_id(soil_id, &self.soil_to_fertilizer_map);
        let water_id = Self::map_id(fertilizer_id, &self.fertilizer_to_water_map);
        let light_id = Self::map_id(water_id, &self.water_to_light_map);

        let temperature_id = Self::map_id(light_id, &self.light_to_temperature_map);
        let humidity_id = Self::map_id(temperature_id, &self.temperature_to_humidity_map);
        let location_id = Self::map_id(humidity_id, &self.humidity_to_location_map);

        location_id
    }

    fn map_id(source_id: i64, map: &Vec<[i64; 3]>) -> i64 {
        for [dest_start, source_start, length] in map {
            //let dest_range = *dest_start..=(*dest_start + *length);
            let source_range = *source_start..=(*source_start + *length);

            if source_range.contains(&source_id) {
                let diff = dest_start - source_start;
                return source_id + diff;
            }
        }

        source_id
    }
}

fn parse_seed_ids<'a>(text: &'a str) -> IResult<&'a str, Vec<i64>> {
    preceded(
        preceded(
            take_until("seeds: "),
            tag("seeds: ")),
        separated_list1(
            space1,
            map(digit1, |nums: &str| nums.parse::<i64>().unwrap()))
    )(text)
}

fn parse_map<'a>(text: &'a str, name: &str) -> IResult<&'a str, Vec<[i64; 3]>> {
    preceded(
        preceded(
            take_until(name),
            terminated(tag(name), tag(" map:"))),
        preceded(
            take_till(|c: char| c.is_ascii_digit()),
            separated_list1(
                newline,
                map(
                    separated_list1(
                        space1,
                        map(
                            digit1,
                            |nums: &str| nums.parse::<i64>().unwrap())),
                    |n: Vec<i64>| [n[0], n[1], n[2]]))))(text)
}

fn parse_seed_data(text: &str) -> SeedMapData {
    let (map_data, seed_ids) = parse_seed_ids(text).unwrap();
    let (map_data, seed_to_soil_map) = parse_map(map_data, "seed-to-soil").unwrap();
    let (map_data, soil_to_fertilizer_map) = parse_map(map_data, "soil-to-fertilizer").unwrap();
    let (map_data, fertilizer_to_water_map) = parse_map(map_data, "fertilizer-to-water").unwrap();
    let (map_data, water_to_light_map) = parse_map(map_data, "water-to-light").unwrap();
    let (map_data, light_to_temperature_map) = parse_map(map_data, "light-to-temperature").unwrap();
    let (map_data, temperature_to_humidity_map) = parse_map(map_data, "temperature-to-humidity").unwrap();
    let (_, humidity_to_location_map) = parse_map(map_data, "humidity-to-location").unwrap();

    SeedMapData {
        seed_ids,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map
    }
}

fn find_lowest_location_number(data: &str) -> i64 {
    let mut smallest_location_id = i64::MAX;
    let seed_data = parse_seed_data(data);

    for seed_id in seed_data.seed_ids.iter() {
        let location_id = seed_data.map_seed_id_to_location(*seed_id);
        //println!("seed id: {seed_id}, location id: {location_id}");

        smallest_location_id = smallest_location_id.min(location_id);
    }

    smallest_location_id
}

fn find_lowest_location_number_with_seed_range(data: &str) -> i64 {
    use rayon::prelude::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, Ordering};

    //let mut smallest_location_id = i64::MAX;
    let seed_data = parse_seed_data(data);
    let smallest_location_id = AtomicI64::new(i64::MAX);

    let seed_id_ranges = seed_data.seed_ids
        .chunks_exact(2)
        .map(|s| s[0]..=(s[0] + s[1]))
        .collect::<Vec<_>>();

    /*for seed_id in seed_id_ranges.into_iter().flatten() {
        let location_id = seed_data.map_seed_id_to_location(seed_id);

        smallest_location_id = smallest_location_id.min(location_id);
    }*/

    seed_id_ranges
        .into_par_iter()
        .flatten()
        .for_each(|seed_id| {
            let location_id = seed_data.map_seed_id_to_location(seed_id);

            //let current_smallest_id = smallest_location_id.;
            //let min_id = smallest_location_id.load(Ordering::Acquire);
            //smallest_location_id.store(min_id.min(location_id), Ordering::Release);

            smallest_location_id.fetch_update(Ordering::Release, Ordering::Acquire, |current_smallest| {
                Some(current_smallest.min(location_id))
            }).unwrap();
        });

    smallest_location_id.into_inner()
}

#[cfg(test)] mod data;

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 35)]
    #[case(TEST_DATA_1, 289863851)]
    fn find_lowest_location_number_test(#[case] data: &str, #[case] expected: i64) {
        let actual = find_lowest_location_number(data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 46)]
    #[case(TEST_DATA_1, 60568880)]
    fn find_lowest_location_number_with_seed_range_test(#[case] data: &str, #[case] expected: i64) {
        let actual = find_lowest_location_number_with_seed_range(data);

        assert_eq!(expected, actual);
    }
}