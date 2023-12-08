use std::{collections::HashMap, ops::Range, io::{BufReader, BufRead}, fs::File};

use crate::range;

const ENTRY_MAP: &str = "seed";

pub struct Manager
{
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    maps: HashMap<String, Map>
}

pub struct Map
{
    #[allow(dead_code)]
    src: String,
    dst: String,

    ranges: Vec<(Range<u64>, Range<u64>)>
}

impl Manager
{
    pub fn from_input(buffer: BufReader<File>) -> Self
    {
        // First line is see
        let lines = buffer.lines();

        let mut manager = Manager {
            seeds: vec![],
            seed_ranges: vec![],
            maps: HashMap::new()
        };

        // Initialize with an unused entry
        let mut current_map = String::from("");

        for line in lines
        {
            match line
            {
                Ok(content) =>
                {
                    // We skip empty lines
                    if content.is_empty() { continue }

                    // First line
                    if content.starts_with("seeds:")
                    {
                        let seeds_content = content.replace("seeds:", "");
                        let seeds_content = seeds_content.trim();

                        // Part 1
                        manager.seeds = seeds_content
                            .split(' ')
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect();

                        // Part 2
                        let mut seed_iter = seeds_content.split(' ');
                        while let Some(seed) = seed_iter.next()
                        {
                            let seed_range_start = seed.parse::<u64>().unwrap();
                            let size = seed_iter.next().unwrap().parse::<u64>().unwrap();

                            manager.seed_ranges.push(seed_range_start..(seed_range_start + size));
                        }
                    }
                    // Header for map
                    else if content.ends_with("map:")
                    {
                        // xxx-to-yyy
                        let mut map_parts = content.split(' ').next().unwrap().split("-to-");
                        let map_src = map_parts.next().unwrap();
                        let map_dst = map_parts.next().unwrap();

                        current_map = map_src.to_string();

                        manager.maps.insert(map_src.to_string(), Map {
                            src: map_src.to_string(),
                            dst: map_dst.to_string(),

                            ranges: vec![]
                        });
                    }
                    // Map line
                    else
                    {
                        let mut range_parts = content
                            .split(' ')
                            .map(|n| n.parse::<u64>().unwrap());

                        let dst_range_start = range_parts.next().unwrap();
                        let src_range_start = range_parts.next().unwrap();
                        let size            = range_parts.next().unwrap();

                        manager.maps.entry(current_map.clone())
                            .and_modify(|e| {
                                e.ranges.push((
                                    src_range_start..(src_range_start + size),
                                    dst_range_start..(dst_range_start + size)
                                ));
                            });
                    }
                },
                Err(e) => println!("Error while reading file: {:?}", e)
            }
        }

        return manager;
    }

    pub fn lowest_location(&self) -> u64
    {
        let mut lowest_location = u64::MAX;

        for seed in &self.seeds
        {
            lowest_location = std::cmp::min(lowest_location, self.get_mapped_value(*seed, "location", ENTRY_MAP));
        }

        return lowest_location
    }

    pub fn lowest_ranges_location(&self) -> u64
    {
        let mut lowest_location = u64::MAX;

        for seed_range in &self.seed_ranges
        {
            lowest_location = lowest_location.min(self.lowest_range_location(seed_range))
        }

        return lowest_location;
    }

    pub fn lowest_range_location(&self, range: &Range::<u64>) -> u64
    {
        let mut translated_ranges = vec![range.clone()];
        let mut current_map = String::from(ENTRY_MAP);

        while let Some(map) = self.maps.get(&current_map)
        {
            current_map = map.dst.clone();

            translated_ranges = translated_ranges
                .into_iter()
                .flat_map(|range| map.translate(range))
                .collect();
        }

        return translated_ranges
            .iter()
            .map(|r| r.start)
            .min()
            .unwrap_or(u64::MAX);
    }

    fn get_mapped_value(&self, value: u64, dst_map: &str, src_map: &str) -> u64
    {
        if dst_map == src_map { return value }

        let mapped_value  = match self.maps.get(src_map)
        {
            Some(map) => (map.dst.as_str(), map.value_for(value)),
            None => panic!("Unknown map {0}", src_map)
        };

        return self.get_mapped_value(mapped_value.1, dst_map, mapped_value.0)
    }
}

impl Map
{
    pub fn value_for(&self, src_value: u64) -> u64
    {
        for range_tuple in &self.ranges
        {
            if range_tuple.0.contains(&src_value)
            {
                let shift = src_value - range_tuple.0.start;
                return range_tuple.1.start + shift;
            }
        }

        return src_value;
    }

    pub fn translate(&self, initial_range: Range<u64>) -> Vec<Range<u64>>
    {
        let mut translated = vec![initial_range.clone()];
        let mut output = vec![];

        while let Some(range) = translated.pop()
        {
            // If we don't find intersect -> non-mapped ranges
            let mut found_intersect = false;

            for mapped_range in &self.ranges
            {
                // If we have no intesection, skip
                if !range::has_intersect(&range, &mapped_range.0) { continue }

                found_intersect = true;

                // We have an intersection, so compute interest range
                let intersect = range::intersect(&range, &mapped_range.0);

                // intersect_start - range.start = shift
                let translated_range = (intersect.start - mapped_range.0.start + mapped_range.1.start)..(intersect.end - mapped_range.0.start + mapped_range.1.start);

                // We add intersection to output
                output.push(translated_range);

                // We also need to re-add remaining intersect (before/after) to find non-mapped values (holes)
                if intersect.start > range.start
                {
                    translated.push(range.start..intersect.start);
                }

                if intersect.end < range.end
                {
                    translated.push(intersect.end..range.end);
                }

                // We break to pop remaining ranges
                break;
            }

            if !found_intersect
            {
                output.push(range);
            }
        }

        return output
    }
}