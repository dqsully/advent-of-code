use std::cmp::{min, max};

use crate::{error::Error, shared::Almanac};

pub fn run(input: &str) -> Result<String, Error> {
    let mut almanac = Almanac::parse(input);

    // Convert every pair of seed numbers into ranges
    let mut seed_ranges = almanac.seeds.windows(2).step_by(2)
        .map(|i| i[0]..i[0]+i[1])
        .collect::<Vec<_>>();

    // Merge mappings from downstream (dest) to upstream (source) to create one
    // precomputed mapping
    let mut combined = almanac.humidity_to_location
        .merge(&mut almanac.temperature_to_humidity)
        .merge(&mut almanac.light_to_temperature)
        .merge(&mut almanac.water_to_light)
        .merge(&mut almanac.fertilizer_to_water)
        .merge(&mut almanac.soil_to_fertilizer)
        .merge(&mut almanac.seed_to_soil);

    // Sort the seed ranges (start/end doesn't matter because there's no overlaps)
    seed_ranges.sort_unstable_by_key(|r| r.end);

    // Sort the mappings by dest so we can start with the smallest mapped dests
    combined.sort_by_dest();

    let mut smallest_mapped = None;

    for mapping in &combined.0 {
        // Find the first seed range that ends after the mapping starts
        let i = seed_ranges.partition_point(|r| r.end <= mapping.source.start);

        if i == seed_ranges.len() {
            // All seed ranges end before mapping starts
            continue;
        }

        // r may overlap with mapping, and if it does, it's the first one
        // that does
        let r = &seed_ranges[i];

        if r.start >= mapping.source.end {
            // Seed range skipped entire mapping, no overlap
            continue;
        }

        // r overlaps, find least common number in seed range and mapping
        // source, then map it
        smallest_mapped = Some(mapping.map(max(r.start, mapping.source.start)));
        break;
    }

    // Sort the mappings by source so that we can find gaps in the sources
    combined.sort_by_source();

    let mut smallest_unmapped = None;

    'unmapped_search: for mut r in seed_ranges.iter().cloned() {
        // Find the first mapping that starts after the seed range's start
        let mut i = combined.0.partition_point(|m| m.source.start <= r.start);

        // Go to the last mapping that starts before or at the seed range's
        // start (if any)
        i = i.saturating_sub(1);

        // Iterate through mappings starting at i until mapping start is after
        // seed range's end
        for mapping in &combined.0[i..] {
            if mapping.source.start >= r.end {
                break;
            }

            if mapping.source.start < r.start && mapping.source.end >= r.start {
                // Mapping overlaps start of seed range, update seed range to
                // start at mapping's end
                r.start = mapping.source.end;
            } else {
                // Mapping doesn't overlap start of seed range, meaning start of
                // seed range is smallest unmapped number
                smallest_unmapped = Some(r.start);
                break 'unmapped_search;
            }
        }
    }

    match (smallest_mapped, smallest_unmapped) {
        (Some(mapped), Some(unmapped)) => Ok(min(mapped, unmapped)),
        (Some(mapped), None) => Ok(mapped),
        (None, Some(unmapped)) => Ok(unmapped),
        (None, None) => Err(Error::NoSmallestFound),
    }.map(|n| n.to_string())
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

        assert_eq!(output, Ok("46".to_owned()));
    }
}
