use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let almanac = Almanac::parse(input);

    let min: usize = almanac.seeds.iter().cloned()
        .map(|n| almanac.seed_to_soil.convert(n))
        .map(|n| almanac.soil_to_fertilizer.convert(n))
        .map(|n| almanac.fertilizer_to_water.convert(n))
        .map(|n| almanac.water_to_light.convert(n))
        .map(|n| almanac.light_to_temperature.convert(n))
        .map(|n| almanac.temperature_to_humidity.convert(n))
        .map(|n| almanac.humidity_to_location.convert(n))
        .min().unwrap();

    Ok(min.to_string())
}

#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    range_len: usize,
}

impl Mapping {
    fn parse(line: &str) -> Mapping {
        let results = line
            .trim()
            .split(' ')
            .filter(|txt| !txt.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(results.len(), 3);

        Mapping {
            dest_start: results[0],
            source_start: results[1],
            range_len: results[2],
        }
    }
}

#[derive(Default, Debug)]
struct Mappings(Vec<Mapping>);

impl Mappings {
    fn parse(contents: &str) -> Mappings {
        Mappings(contents.lines().map(Mapping::parse).collect::<Vec<_>>())
    }

    fn convert(&self, source: usize) -> usize {
        for mapping in &self.0 {
            if (mapping.source_start .. mapping.source_start + mapping.range_len).contains(&source) {
                return source - mapping.source_start + mapping.dest_start;
            }
        }

        source
    }
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<usize>,

    seed_to_soil: Mappings,
    soil_to_fertilizer: Mappings,
    fertilizer_to_water: Mappings,
    water_to_light: Mappings,
    light_to_temperature: Mappings,
    temperature_to_humidity: Mappings,
    humidity_to_location: Mappings,
}

impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut almanac: Almanac = Default::default();

        for section in input.split("\n\n") {
            let (header, contents) = section.split_once(":").unwrap();
            let contents = contents.trim();

            if header == "seeds" {
                almanac.seeds = contents.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
            } else if header.ends_with(" map") {
                let map_type = header.split_once(' ').unwrap().0;

                let mappings = Mappings::parse(contents);

                match map_type {
                    "seed-to-soil" => almanac.seed_to_soil = mappings,
                    "soil-to-fertilizer" => almanac.soil_to_fertilizer = mappings,
                    "fertilizer-to-water" => almanac.fertilizer_to_water = mappings,
                    "water-to-light" => almanac.water_to_light = mappings,
                    "light-to-temperature" => almanac.light_to_temperature = mappings,
                    "temperature-to-humidity" => almanac.temperature_to_humidity = mappings,
                    "humidity-to-location" => almanac.humidity_to_location = mappings,
                    _ => panic!("unknown map type {map_type}"),
                };
            } else {
                panic!("invalid header {header}");
            }
        }

        almanac
    }
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
