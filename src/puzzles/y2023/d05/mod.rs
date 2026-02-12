use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

struct RangeMap(Vec<TransformRange>);

impl From<&str> for RangeMap {
    fn from(value: &str) -> Self {
        let mut ranges = value
            .lines()
            .skip(1)
            .map(TransformRange::from)
            .collect::<Vec<_>>();
        ranges.sort();
        Self(ranges)
    }
}

impl RangeMap {
    fn resolve_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut new = vec![];

        for range in ranges {
            let mut splits = vec![TransformRange { range, diff: 0 }];
            for transform in self.0.iter() {
                let to_split = splits.pop().unwrap();

                if transform.range.contains_range(&to_split.range) {
                    splits.push(TransformRange {
                        range: to_split.range,
                        diff: transform.diff,
                    });
                    break;
                } else if to_split.range.contains_range(&transform.range) {
                    if transform.range.start > to_split.range.start {
                        splits.push(TransformRange {
                            range: Range {
                                start: to_split.range.start,
                                end: transform.range.start,
                            },
                            diff: 0,
                        });
                    }
                    splits.push(TransformRange {
                        range: Range {
                            start: transform.range.start,
                            end: transform.range.end,
                        },
                        diff: transform.diff,
                    });
                    if to_split.range.end > transform.range.end {
                        splits.push(TransformRange {
                            range: Range {
                                start: transform.range.end,
                                end: to_split.range.end,
                            },
                            diff: 0,
                        });
                    } else {
                        break;
                    }
                } else if transform.range.contains(to_split.range.start) {
                    // Note: `transform.range` cannot also contain `to_split.range.end`
                    splits.push(TransformRange {
                        range: Range {
                            start: to_split.range.start,
                            end: transform.range.end,
                        },
                        diff: transform.diff,
                    });
                    splits.push(TransformRange {
                        range: Range {
                            start: transform.range.end,
                            end: to_split.range.end,
                        },
                        diff: 0,
                    });
                } else if transform.range.contains(to_split.range.end) {
                    // Note: `transform.range` cannot also contain `to_split.range.start`
                    splits.push(TransformRange {
                        range: Range {
                            start: to_split.range.start,
                            end: transform.range.start,
                        },
                        diff: 0,
                    });
                    splits.push(TransformRange {
                        range: Range {
                            start: transform.range.start,
                            end: to_split.range.end,
                        },
                        diff: transform.diff,
                    });
                    break;
                } else {
                    splits.push(to_split);
                }
            }

            for split in splits {
                new.push(Range {
                    start: split.range.start + split.diff,
                    end: split.range.end + split.diff,
                });
            }
        }

        new.sort();
        new
    }
}

#[derive(Eq, PartialEq)]
struct TransformRange {
    range: Range,
    diff: isize,
}

impl From<&str> for TransformRange {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ");
        let dest = iter.next().unwrap().parse::<isize>().unwrap();
        let src = iter.next().unwrap().parse::<isize>().unwrap();
        let len = iter.next().unwrap().parse::<isize>().unwrap();
        Self {
            range: Range {
                start: src,
                end: src + len,
            },
            diff: dest - src,
        }
    }
}

impl PartialOrd for TransformRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.partial_cmp(&other.range)
    }
}

impl Ord for TransformRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.cmp(&other.range)
    }
}

#[derive(Eq, PartialEq)]
struct Range {
    start: isize,
    end: isize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Range {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start) && self.contains(other.end - 1)
    }

    fn contains(&self, n: isize) -> bool {
        self.start <= n && n < self.end
    }
}

fn solver() -> (String, String) {
    let mut iter = INPUT.split("\n\n");

    let seeds = iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|str| str.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let seed_to_soil = RangeMap::from(iter.next().unwrap());
    let soil_to_fertilizer = RangeMap::from(iter.next().unwrap());
    let fertilizer_to_water = RangeMap::from(iter.next().unwrap());
    let water_to_light = RangeMap::from(iter.next().unwrap());
    let light_to_temperature = RangeMap::from(iter.next().unwrap());
    let temperature_to_humidity = RangeMap::from(iter.next().unwrap());
    let humidity_to_location = RangeMap::from(iter.next().unwrap());

    let min_location_1 = seeds.iter().fold(isize::MAX, |global_min, seed| {
        let seed = vec![Range {
            start: *seed,
            end: seed + 1,
        }];
        let soil = seed_to_soil.resolve_ranges(seed);
        let fertilizer = soil_to_fertilizer.resolve_ranges(soil);
        let water = fertilizer_to_water.resolve_ranges(fertilizer);
        let light = water_to_light.resolve_ranges(water);
        let temperature = light_to_temperature.resolve_ranges(light);
        let humidity = temperature_to_humidity.resolve_ranges(temperature);
        let location = humidity_to_location.resolve_ranges(humidity);

        let local_min = location.iter().map(|l| l.start).min().unwrap();
        global_min.min(local_min)
    });

    let min_location_2 = seeds.chunks(2).fold(isize::MAX, |global_min, pair| {
        let seed = vec![Range {
            start: pair[0],
            end: pair[0] + pair[1],
        }];

        let soil = seed_to_soil.resolve_ranges(seed);
        let fertilizer = soil_to_fertilizer.resolve_ranges(soil);
        let water = fertilizer_to_water.resolve_ranges(fertilizer);
        let light = water_to_light.resolve_ranges(water);
        let temperature = light_to_temperature.resolve_ranges(light);
        let humidity = temperature_to_humidity.resolve_ranges(temperature);
        let location = humidity_to_location.resolve_ranges(humidity);

        let local_min = location.iter().map(|l| l.start).min().unwrap();
        global_min.min(local_min)
    });

    (min_location_1.to_string(), min_location_2.to_string())
}
