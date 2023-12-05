use crate::{error::Error, shared::Almanac};

pub fn run(input: &str) -> Result<String, Error> {
    let mut almanac = Almanac::parse(input);

    let mut combined = almanac.humidity_to_location
        .merge(&mut almanac.temperature_to_humidity)
        .merge(&mut almanac.light_to_temperature)
        .merge(&mut almanac.water_to_light)
        .merge(&mut almanac.fertilizer_to_water)
        .merge(&mut almanac.soil_to_fertilizer)
        .merge(&mut almanac.seed_to_soil);

    combined.sort_by_source();

    let min: usize = almanac.seeds.iter().copied()
        .map(|seed| combined.map(seed))
        .min().unwrap();

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let output = run(input);

        assert_eq!(output, Ok("35".to_owned()));
    }
}
