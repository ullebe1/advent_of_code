use std::ops::Range;
use std::slice::Iter;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let binding = input.lines().collect::<Vec<&str>>();
    let mut lines = binding.iter();
    
    // Get seeds
    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();
    let seeds = seeds.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    
    // Get seed-to-soil map:
    lines.next();
    lines.next();
    let (seed_to_soil, mut lines) = get_ranges(&mut lines);
    
    // Get soil-to-fertilizer map:
    lines.next();
    lines.next();
    let (soil_to_fertilizer, mut lines) = get_ranges(&mut lines);
    
    // Get fertilizer-to-water map:
    lines.next();
    lines.next();
    let (fertilizer_to_water, mut lines) = get_ranges(&mut lines);
    
    // Get water-to-light map:
    lines.next();
    lines.next();
    let (water_to_light, mut lines) = get_ranges(&mut lines);
    
    // Get light-to-temperature map:
    lines.next();
    lines.next();
    let (light_to_temperature, mut lines) = get_ranges(&mut lines);
    
    // Get temperature-to-humidity map:
    lines.next();
    lines.next();
    let (temperature_to_humidity, mut lines) = get_ranges(&mut lines);
    
    // Get humidity-to-location map:
    lines.next();
    lines.next();
    let (humidity_to_location, _) = get_ranges(&mut lines);
    
    Input {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn get_ranges<'a>(lines: &'a mut Iter<'_, &str>) -> (Vec<(Range<usize>, Range<usize>)>, Iter<'a, &'a str>) {
    let mut ranges = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (destination, source_and_length) = line.split_once(" ").unwrap();
        let (source, length) = source_and_length.split_once(" ").unwrap();
        let destination = destination.parse::<usize>().unwrap();
        let source = source.parse::<usize>().unwrap();
        let length = length.parse::<usize>().unwrap();
        ranges.push((Range { start: destination, end: destination + length }, Range { start: source, end: source + length }));
    }

    (ranges, lines.to_owned())
}

pub fn get_corresponding(number: usize, range1: Range<usize>, range2: Range<usize>) -> usize {
    let index = range1.clone().position(|n| n == number).unwrap();
    range2.clone().nth(index).unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let mut lowest = 0;
    
    for seed in input.seeds.clone() {
        println!("Seed: {}", seed);
        let mut index = seed;
        index = get_corresponding(index, input.seed_to_soil[0].0.clone(), input.seed_to_soil[0].1.clone());
        index = get_corresponding(index, input.soil_to_fertilizer[0].0.clone(), input.soil_to_fertilizer[0].1.clone());
        index = get_corresponding(index, input.fertilizer_to_water[0].0.clone(), input.fertilizer_to_water[0].1.clone());
        index = get_corresponding(index, input.water_to_light[0].0.clone(), input.water_to_light[0].1.clone());
        index = get_corresponding(index, input.light_to_temperature[0].0.clone(), input.light_to_temperature[0].1.clone());
        index = get_corresponding(index, input.temperature_to_humidity[0].0.clone(), input.temperature_to_humidity[0].1.clone());
        index = get_corresponding(index, input.humidity_to_location[0].0.clone(), input.humidity_to_location[0].1.clone());
        if lowest == 0 || index < lowest {
            lowest = index;
        }
    }
    
    lowest
}
