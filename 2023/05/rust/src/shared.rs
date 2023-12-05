use std::cmp::{min, max};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mapping {
    pub source: Range<usize>,
    pub dest: Range<usize>,
}

pub struct Remainders {
    pub before: Option<Mapping>,
    pub after: Option<Mapping>,
}

pub enum MappingSide {
    Source,
    Dest,
}

impl Mapping {
    pub fn parse(line: &str) -> Mapping {
        let results = line
            .trim()
            .split(' ')
            .filter(|txt| !txt.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(results.len(), 3);

        Mapping {
            source: results[1]..results[1]+results[2],
            dest: results[0]..results[0]+results[2],
        }
    }

    pub fn intersection(&self, upstream: &Mapping) -> Option<(Mapping, Remainders, Remainders)> {
        let intersected = max(self.source.start, upstream.dest.start)..min(self.source.end, upstream.dest.end);

        if !intersected.is_empty() {
            let intersection = Mapping {
                source: (intersected.start - upstream.dest.start + upstream.source.start)..(intersected.end - upstream.dest.start + upstream.source.start),
                dest: (intersected.start - self.source.start + self.dest.start)..(intersected.end - self.source.start + self.dest.start),
            };

            Some((intersection, self.subtract(MappingSide::Source, &intersected), upstream.subtract(MappingSide::Dest, &intersected)))
        } else {
            None
        }

    }

    pub fn subtract(&self, side: MappingSide, range: &Range<usize>) -> Remainders {
        let self_range = match side {
            MappingSide::Source => self.source.clone(),
            MappingSide::Dest => self.dest.clone(),
        };

        let mut before = None;

        if self_range.start < range.start && self_range.end > range.start {
            // self overlaps range at start, must be before remainder

            let len = range.start - self_range.start;

            before = Some(Mapping {
                source: self.source.start..self.source.start+len,
                dest: self.dest.start..self.dest.start+len,
            });
        }

        let mut after = None;

        if self_range.end > range.end && self_range.start < range.end {
            // self overlaps range at end, must be after remainder

            let len = self_range.end - range.end;

            after = Some(Mapping {
                source: self.source.end-len..self.source.end,
                dest: self.dest.end-len..self.dest.end,
            });
        }

        Remainders {
            before,
            after,
        }
    }

    pub fn map(&self, source: usize) -> usize {
        if self.source.contains(&source) {
            return source - self.source.start + self.dest.start;
        }

        source
    }
}

#[derive(Default, Debug)]
pub struct Mappings(pub Vec<Mapping>);

impl Mappings {
    pub fn parse(contents: &str) -> Mappings {
        Mappings(contents.lines().map(Mapping::parse).collect())
    }

    pub fn sort_by_source(&mut self) {
        self.0.sort_unstable_by_key(|m| m.source.start)
    }

    pub fn sort_by_dest(&mut self) {
        self.0.sort_unstable_by_key(|m| m.dest.start)
    }

    pub fn merge(&mut self, upstream: &mut Mappings) -> Mappings {
        // Map upstream dests to downstream sources and merge when intersecting
        upstream.sort_by_dest();
        self.sort_by_source();

        let mut down_iter = self.0.iter().cloned();
        let mut up_iter = upstream.0.iter().cloned();

        let mut down_value = down_iter.next();
        let mut up_value = up_iter.next();

        let mut out = Vec::new();

        // Compute upstream dest and downstream source intersections
        while let (Some(down_map), Some(up_map)) = (&mut down_value, &mut up_value) {
            if let Some((intersection, down_rem, up_rem)) = down_map.intersection(up_map) {
                // upstream dest intersects with downstream source

                if let Some(down_before) = down_rem.before {
                    out.push(down_before);
                } else if let Some(up_before) = up_rem.before {
                    out.push(up_before);
                }

                out.push(intersection);

                if let Some(down_after) = down_rem.after {
                    // downstream source ends after upstream dest
                    *down_map = down_after;
                    up_value = up_iter.next();
                } else if let Some(up_after) = up_rem.after {
                    // upstream dest ends after downstream source
                    *up_map = up_after;
                    down_value = down_iter.next();
                } else {
                    // upstream dest and downstream source ended at the same place
                    down_value = down_iter.next();
                    up_value = up_iter.next();
                }
            } else if down_map.source.start < up_map.dest.start {
                // downstream source starts before upstream dest but neither
                // overlap, add and get next downstream mapping

                out.push(down_map.clone());
                down_value = down_iter.next();
            } else {
                // upstream dest starts before downstream source but neither
                // overlap, add and get next upstream mapping

                out.push(up_map.clone());
                up_value = up_iter.next();
            }
        }

        // Append any remaining upstream/downstream mappings
        if down_value.is_some() {
            while let Some(down_map) = &down_value {
                out.push(down_map.clone());
                down_value = down_iter.next();
            }
        } else {
            while let Some(up_map) = &up_value {
                out.push(up_map.clone());
                up_value = up_iter.next();
            }
        }

        Mappings(out)
    }

    /// **MUST be sorted by source!**
    pub fn map(&self, source: usize) -> usize {
        let i = self.0.partition_point(|m| m.source.start <= source);

        if i > 0 {
            self.0[i - 1].map(source)
        } else {
            source
        }
    }
}

#[derive(Default, Debug)]
pub struct Almanac {
    pub seeds: Vec<usize>,

    pub seed_to_soil: Mappings,
    pub soil_to_fertilizer: Mappings,
    pub fertilizer_to_water: Mappings,
    pub water_to_light: Mappings,
    pub light_to_temperature: Mappings,
    pub temperature_to_humidity: Mappings,
    pub humidity_to_location: Mappings,
}

impl Almanac {
    pub fn parse(input: &str) -> Almanac {
        let mut almanac: Almanac = Default::default();

        for section in input.split("\n\n") {
            let (header, contents) = section.split_once(':').unwrap();
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
