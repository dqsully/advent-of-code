use crate::{error::Error, shared::{Almanac, Mapping, Mappings}};

pub fn run(input: &str) -> Result<String, Error> {
    let mut almanac = Almanac::parse(input);

    // Convert every seed into a 1-sized range
    let seed_ranges = almanac
        .seeds
        .iter()
        .map(|&i| i..i+1)
        .map(|r| Mapping{source: r.clone(), dest: r})
        .collect::<Vec<_>>();

    // Merge mappings from upstream (source) to downstream (dest) to create one
    // precomputed seed-to-location mapping
    let mut combined = Mappings(seed_ranges)
        .merge(&mut almanac.seed_to_soil)
        .merge(&mut almanac.soil_to_fertilizer)
        .merge(&mut almanac.fertilizer_to_water)
        .merge(&mut almanac.water_to_light)
        .merge(&mut almanac.light_to_temperature)
        .merge(&mut almanac.temperature_to_humidity)
        .merge(&mut almanac.humidity_to_location);

    // Sort the mappings by dest so we can start with the smallest mapped dests
    combined.sort_by_dest();

    combined.0
        .first()
        .ok_or(Error::NoSmallestFound)
        .map(|m| m.dest.start.to_string())
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
