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
    pub seed_ids: Vec<u32>,
    pub seed_to_soil_map: Vec<[u32; 3]>,
    pub soil_to_fertilizer_map: Vec<[u32; 3]>,
    pub fertilizer_to_water_map: Vec<[u32; 3]>,
    pub water_to_light_map: Vec<[u32; 3]>,
    pub light_to_temperature_map: Vec<[u32; 3]>,
    pub temperature_to_humidity_map: Vec<[u32; 3]>,
    pub humidity_to_location_map: Vec<[u32; 3]>,
}

fn parse_seed_ids<'a>(text: &'a str) -> IResult<&'a str, Vec<u32>> {
    preceded(
        preceded(
            take_until("seeds: "),
            tag("seeds: ")),
        separated_list1(
            space1,
            map(digit1, |nums: &str| nums.parse::<u32>().unwrap()))
    )(text)
}

fn parse_map<'a>(text: &'a str, name: &str) -> IResult<&'a str, Vec<[u32; 3]>> {
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
                            |nums: &str| nums.parse::<u32>().unwrap())),
                    |n: Vec<u32>| [n[0], n[1], n[2]]))))(text)
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

fn find_lowest_location_number(data: &str) -> u32 {
    let seed_data = parse_seed_data(data);

    //println!("{seed_data:#?}\n\n\n");

    0
}

#[cfg(test)] mod data;

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 35)]
    fn find_lowest_location_number_test(#[case] data: &str, #[case] expected: u32) {
        let actual = find_lowest_location_number(data);

        assert_eq!(expected, actual);
    }
}